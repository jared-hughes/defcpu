use std::ops::BitXor;

#[derive(PartialEq, Eq)]
pub enum U1 {
    Zero,
    One,
}
impl From<bool> for U1 {
    fn from(value: bool) -> Self {
        match value {
            false => U1::Zero,
            true => U1::One,
        }
    }
}
impl From<U1> for bool {
    fn from(value: U1) -> bool {
        match value {
            U1::Zero => false,
            U1::One => true,
        }
    }
}
impl U1 {
    pub fn as_u8(&self) -> u8 {
        match self {
            U1::Zero => 0,
            U1::One => 1,
        }
    }
}

impl BitXor for U1 {
    type Output = U1;

    fn bitxor(self, rhs: Self) -> Self::Output {
        (self != rhs).into()
    }
}

#[derive(PartialEq, Eq)]
pub enum Sign {
    Neg,
    Pos,
}
impl From<U1> for Sign {
    fn from(value: U1) -> Self {
        match value {
            U1::Zero => Sign::Pos,
            U1::One => Sign::Neg,
        }
    }
}
impl From<Sign> for U1 {
    fn from(value: Sign) -> U1 {
        match value {
            Sign::Pos => U1::Zero,
            Sign::Neg => U1::One,
        }
    }
}
