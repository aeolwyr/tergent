use std::convert::TryFrom;
use std::os::raw::c_ulong;

use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

pub const _VALUE: u32 = 0;
pub const _PIN: u32 = 1;
pub const _CHALLENGE: u32 = 2;
pub const _TIME: u32 = 3;
pub const _COUNTER: u32 = 4;
pub const _FLAGS: u32 = 5;
pub const _OUTPUT_LENGTH: u32 = 6;
pub const _OUTPUT_FORMAT: u32 = 7;

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum Format {
    Decimal = 0,
    Hexadecimal = 1,
    Alphanumeric = 2,
    Binary = 3,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum Param {
    Ignored = 0,
    Optional = 1,
    Mandatory = 2,
}

impl TryFrom<c_ulong> for Format {
    type Error = ();
    fn try_from(value: c_ulong) -> Result<Self, Self::Error> {
        Format::from_u64(value).ok_or(())
    }
}

impl TryFrom<Format> for c_ulong {
    type Error = ();
    fn try_from(value: Format) -> Result<Self, Self::Error> {
        Format::to_u64(&value).ok_or(())
    }
}

impl TryFrom<c_ulong> for Param {
    type Error = ();
    fn try_from(value: c_ulong) -> Result<Self, Self::Error> {
        Param::from_u64(value).ok_or(())
    }
}

impl TryFrom<Param> for c_ulong {
    type Error = ();
    fn try_from(value: Param) -> Result<Self, Self::Error> {
        Param::to_u64(&value).ok_or(())
    }
}
