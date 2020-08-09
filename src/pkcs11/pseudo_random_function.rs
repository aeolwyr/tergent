use std::convert::TryFrom;
use std::os::raw::c_ulong;

use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum PseudoRandomFunction {
    Pkcs5Pbkd2HmacSha1 = 1,
    Pkcs5Pbkd2HmacGostr3411 = 2,
    Pkcs5Pbkd2HmacSha224 = 3,
    Pkcs5Pbkd2HmacSha256 = 4,
    Pkcs5Pbkd2HmacSha384 = 5,
    Pkcs5Pbkd2HmacSha512 = 6,
    Pkcs5Pbkd2HmacSha512224 = 7,
    Pkcs5Pbkd2HmacSha512256 = 8,
}

impl TryFrom<c_ulong> for PseudoRandomFunction {
    type Error = ();
    fn try_from(value: c_ulong) -> Result<Self, Self::Error> {
        PseudoRandomFunction::from_u64(value).ok_or(())
    }
}

impl TryFrom<PseudoRandomFunction> for c_ulong {
    type Error = ();
    fn try_from(value: PseudoRandomFunction) -> Result<Self, Self::Error> {
        PseudoRandomFunction::to_u64(&value).ok_or(())
    }
}
