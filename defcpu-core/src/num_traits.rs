// This is not the num_traits crate.

use std::ops::{
    Add, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div, Mul, Not, Rem, Shl,
    ShlAssign, Shr, ShrAssign, Sub,
};

use crate::num_u1::{Sign, U1};

pub trait HasDoubleWidth: Sized {
    type DoubleWidth: UNum + HasHalfWidth<Self>;

    /// Send to double width, zero-extending.
    fn zero_extend_double_width(&self) -> Self::DoubleWidth;

    /// Send to double width, sign-extending.
    fn sign_extend_double_width(&self) -> Self::DoubleWidth;
}

pub trait HasHalfWidth<HalfWidth>: Sized {
    /// Discard higher bits, keeping the lower half of the bits.
    fn truncate_to_half_width(&self) -> HalfWidth;

    fn try_into_half_width(&self) -> Result<HalfWidth, ()>;
}

macro_rules! double_width_impl {
    ($t:ty, $i:ty, $td:ty) => {
        impl HasDoubleWidth for $t {
            type DoubleWidth = $td;

            fn zero_extend_double_width(&self) -> $td {
                *self as $td
            }

            fn sign_extend_double_width(&self) -> $td {
                *self as $i as $td
            }
        }
        impl HasHalfWidth<$t> for $td {
            fn truncate_to_half_width(&self) -> $t {
                *self as $t
            }

            fn try_into_half_width(&self) -> Result<$t, ()> {
                let x = self.truncate_to_half_width();
                if (x as $td) == *self {
                    Ok(x)
                } else {
                    Err(())
                }
            }
        }
    };
}

double_width_impl!(u8, i8, u16);
double_width_impl!(u16, i16, u32);
double_width_impl!(u32, i32, u64);
double_width_impl!(u64, i64, u128);

pub trait UNum8To64: UNum + HasDoubleWidth + WideningSignedMul {}

impl UNum8To64 for u8 {}
impl UNum8To64 for u16 {}
impl UNum8To64 for u32 {}
impl UNum8To64 for u64 {}

/// UNum: u8, u16, u32, u64, u128
pub trait UNum:
    Add<Output = Self>
    + OverflowingAdd
    + OverflowingSub
    + Div<Output = Self>
    + Mul<Output = Self>
    + Rem<Output = Self>
    + Sub<Output = Self>
    + Not<Output = Self>
    + BitAnd<Output = Self>
    + BitOr<Output = Self>
    + BitXor<Output = Self>
    + Shl<Output = Self>
    + Shr<u32, Output = Self>
    + RotateLeft
    + RotateRight
    + BitAndAssign
    + BitOrAssign
    + BitXorAssign
    + ShlAssign
    + ShrAssign
    + Sized
    + Clone
    + Copy
    + From<u8>
    + TryInto<u8>
    + Eq
{
    const BITS: u32;
    const MIN: Self;
    const MAX: Self;

    /// The `n` th bit, where the 0th bit is the least-significant bit.
    /// If `n` is at least Self::BITS, gives 0.
    #[inline]
    fn bit(&self, n: u32) -> U1 {
        (*self >> n & 1.into() == 1.into()).into()
    }

    /// Get the high bit.
    #[inline]
    fn msb_bit(&self) -> U1 {
        self.bit(Self::BITS - 1)
    }

    /// Get the high bit, interpreted as a sign.
    #[inline]
    fn sign(&self) -> Sign {
        self.msb_bit().into()
    }

    /// Get the lower byte.
    #[inline]
    fn as_u8(&self) -> u8 {
        // Godbolt verifies Rust is smart enough to do this same as `self as u8` in release.
        // Writing it this way because there's no free trait for `as u8`.
        (*self & 0xFF.into())
            .try_into()
            .unwrap_or_else(|_| panic!("0xFF should fit in u8."))
    }
}

impl UNum for u8 {
    const BITS: u32 = 8;
    const MIN: Self = Self::MIN;
    const MAX: Self = Self::MAX;
}

impl UNum for u16 {
    const BITS: u32 = 16;
    const MIN: Self = Self::MIN;
    const MAX: Self = Self::MAX;
}

impl UNum for u32 {
    const BITS: u32 = 32;
    const MIN: Self = Self::MIN;
    const MAX: Self = Self::MAX;
}

impl UNum for u64 {
    const BITS: u32 = 64;
    const MIN: Self = Self::MIN;
    const MAX: Self = Self::MAX;
}

impl UNum for u128 {
    const BITS: u32 = 128;
    const MIN: Self = Self::MIN;
    const MAX: Self = Self::MAX;
}

macro_rules! overflowing_method_impl {
    ($trait_name:ident, $method:ident, $t:ty) => {
        impl $trait_name for $t {
            #[inline]
            fn $method(&self, rhs: &Self) -> (Self, bool) {
                <$t>::$method(*self, *rhs)
            }
        }
    };
}

pub trait OverflowingAdd: Sized + Add<Self, Output = Self> {
    /// Calculates `self` + `rhs`
    ///
    /// Returns a tuple of the addition along with a boolean indicating
    /// whether an arithmetic overflow would occur. If an overflow would
    /// have occurred then the wrapped value is returned.
    fn overflowing_add(&self, rhs: &Self) -> (Self, bool);
}

overflowing_method_impl!(OverflowingAdd, overflowing_add, u8);
overflowing_method_impl!(OverflowingAdd, overflowing_add, u16);
overflowing_method_impl!(OverflowingAdd, overflowing_add, u32);
overflowing_method_impl!(OverflowingAdd, overflowing_add, u64);
overflowing_method_impl!(OverflowingAdd, overflowing_add, u128);

pub trait OverflowingSub: Sized + Sub<Self, Output = Self> {
    /// Calculates `self` - `rhs`
    ///
    /// Returns a tuple of the subtraction along with a boolean indicating
    /// whether an arithmetic overflow would occur. If an overflow would
    /// have occurred then the wrapped value is returned.
    fn overflowing_sub(&self, rhs: &Self) -> (Self, bool);
}

overflowing_method_impl!(OverflowingSub, overflowing_sub, u8);
overflowing_method_impl!(OverflowingSub, overflowing_sub, u16);
overflowing_method_impl!(OverflowingSub, overflowing_sub, u32);
overflowing_method_impl!(OverflowingSub, overflowing_sub, u64);
overflowing_method_impl!(OverflowingSub, overflowing_sub, u128);

macro_rules! rotate_method_impl {
    ($trait_name:ident, $method:ident, $t:ty) => {
        impl $trait_name for $t {
            #[inline]
            fn $method(&self, n: u32) -> Self {
                <$t>::$method(*self, n)
            }
        }
    };
}

pub trait RotateLeft: Sized {
    /// Shifts the bits to the left by a specified amount, `n`, wrapping the
    /// truncated bits to the end of the resulting integer.
    fn rotate_left(&self, n: u32) -> Self;
}
rotate_method_impl!(RotateLeft, rotate_left, u8);
rotate_method_impl!(RotateLeft, rotate_left, u16);
rotate_method_impl!(RotateLeft, rotate_left, u32);
rotate_method_impl!(RotateLeft, rotate_left, u64);
rotate_method_impl!(RotateLeft, rotate_left, u128);

pub trait RotateRight: Sized {
    /// Shifts the bits to the left by a specified amount, `n`, wrapping the
    /// truncated bits to the end of the resulting integer.
    #[allow(unused)]
    fn rotate_right(&self, n: u32) -> Self;
}
rotate_method_impl!(RotateRight, rotate_right, u8);
rotate_method_impl!(RotateRight, rotate_right, u16);
rotate_method_impl!(RotateRight, rotate_right, u32);
rotate_method_impl!(RotateRight, rotate_right, u64);
rotate_method_impl!(RotateRight, rotate_right, u128);

pub trait WideningSignedMul: HasDoubleWidth + Sized {
    /// Calculates `self` * `rhs`, with `self` and `rhs` treated as signed,
    /// extended to a double-width value.
    fn widening_signed_mul(&self, rhs: &Self) -> Self::DoubleWidth;
}

macro_rules! widening_signed_mul_impl {
    ($trait_name:ident, $method:ident, $t:ty, $id:ty) => {
        impl $trait_name for $t {
            #[inline]
            fn $method(&self, rhs: &Self) -> Self::DoubleWidth {
                let i_self = self.sign_extend_double_width() as $id;
                let i_rhs = rhs.sign_extend_double_width() as $id;
                (i_self * i_rhs) as Self::DoubleWidth
            }
        }
    };
}

widening_signed_mul_impl!(WideningSignedMul, widening_signed_mul, u8, i16);
widening_signed_mul_impl!(WideningSignedMul, widening_signed_mul, u16, i32);
widening_signed_mul_impl!(WideningSignedMul, widening_signed_mul, u32, i64);
widening_signed_mul_impl!(WideningSignedMul, widening_signed_mul, u64, i128);
