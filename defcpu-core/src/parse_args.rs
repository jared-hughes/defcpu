use std::{fmt, str::FromStr};

#[derive(Debug)]
pub struct Rand16(pub [u8; 16]);

#[derive(Debug)]
pub enum Rand16Err {
    FromHex(hex::FromHexError),
    WrongLen(usize),
    BadFromSlice(std::array::TryFromSliceError),
}
impl fmt::Display for Rand16Err {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Rand16Err::FromHex(e) => write!(f, "{e}"),
            Rand16Err::WrongLen(e) => write!(f, "Need 32 hex digits, but got {}", 2 * e),
            Rand16Err::BadFromSlice(e) => write!(f, "{e}"),
        }
    }
}
impl FromStr for Rand16 {
    type Err = Rand16Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let y = hex::decode(s).map_err(Rand16Err::FromHex)?;
        if y.len() != 16 {
            return Err(Rand16Err::WrongLen(y.len()));
        }
        let tup: &[u8; 16] = y[..].try_into().map_err(Rand16Err::BadFromSlice)?;
        Ok(Rand16(*tup))
    }
}

#[derive(Debug)]
pub struct Hex64(pub u64);

#[derive(Debug)]
pub enum Hex64Err {
    Missing0x,
    ParseInt(std::num::ParseIntError),
}
impl fmt::Display for Hex64Err {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Hex64Err::ParseInt(e) => write!(f, "{e}"),
            Hex64Err::Missing0x => write!(f, "Missing 0x prefix."),
        }
    }
}
impl FromStr for Hex64 {
    type Err = Hex64Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let digs = s.strip_prefix("0x").ok_or(Hex64Err::Missing0x)?;
        let num = u64::from_str_radix(digs, 16).map_err(Hex64Err::ParseInt)?;
        Ok(Hex64(num))
    }
}
