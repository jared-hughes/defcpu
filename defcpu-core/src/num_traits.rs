// This is not the num_traits crate.

use std::ops::{
    Add, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div, Mul, Not, Rem, Shl,
    ShlAssign, Shr, ShrAssign, Sub,
};

use crate::num_u1::{Sign, U1};

pub trait HasDoubleWidth {
    type DoubleWidth: UNum + HasHalfWidth;

    /// Send to double width, zero-extending.
    #[allow(unused)]
    fn zero_extend_double_width(&self) -> Self::DoubleWidth;

    /// Send to double width, sign-extending.
    fn sign_extend_double_width(&self) -> Self::DoubleWidth;
}

pub trait HasHalfWidth {
    type HalfWidth: UNum + HasDoubleWidth;

    /// Discard higher bits, keeping the lower half of the bits.
    fn truncate_to_half_width(&self) -> Self::HalfWidth;
}

macro_rules! double_width_impl {
    ($t:ty, $i:ty, $td:ty) => {
        impl HasDoubleWidth for $t {
            type DoubleWidth = $td;

            fn zero_extend_double_width(&self) -> Self::DoubleWidth {
                *self as $td
            }

            fn sign_extend_double_width(&self) -> Self::DoubleWidth {
                *self as $i as $td
            }
        }
        impl HasHalfWidth for $td {
            type HalfWidth = $t;

            fn truncate_to_half_width(&self) -> Self::HalfWidth {
                *self as $t
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

macro_rules! overflowing_impl {
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

overflowing_impl!(OverflowingAdd, overflowing_add, u8);
overflowing_impl!(OverflowingAdd, overflowing_add, u16);
overflowing_impl!(OverflowingAdd, overflowing_add, u32);
overflowing_impl!(OverflowingAdd, overflowing_add, u64);
overflowing_impl!(OverflowingAdd, overflowing_add, u128);

pub trait OverflowingSub: Sized + Sub<Self, Output = Self> {
    /// Calculates `self` - `rhs`
    ///
    /// Returns a tuple of the subtraction along with a boolean indicating
    /// whether an arithmetic overflow would occur. If an overflow would
    /// have occurred then the wrapped value is returned.
    fn overflowing_sub(&self, rhs: &Self) -> (Self, bool);
}

overflowing_impl!(OverflowingSub, overflowing_sub, u8);
overflowing_impl!(OverflowingSub, overflowing_sub, u16);
overflowing_impl!(OverflowingSub, overflowing_sub, u32);
overflowing_impl!(OverflowingSub, overflowing_sub, u64);
overflowing_impl!(OverflowingSub, overflowing_sub, u128);

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