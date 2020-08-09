use std::convert::TryFrom;
use std::os::raw::c_ulong;

use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum MaskGenerationFunction {
    Mgf1Sha1 = 1,
    Mgf1Sha256 = 2,
    Mgf1Sha384 = 3,
    Mgf1Sha512 = 4,
    Mgf1Sha224 = 5,
}

impl TryFrom<c_ulong> for MaskGenerationFunction {
    type Error = ();
    fn try_from(value: c_ulong) -> Result<Self, Self::Error> {
        MaskGenerationFunction::from_u64(value).ok_or(())
    }
}

impl TryFrom<MaskGenerationFunction> for c_ulong {
    type Error = ();
    fn try_from(value: MaskGenerationFunction) -> Result<Self, Self::Error> {
        MaskGenerationFunction::to_u64(&value).ok_or(())
    }
}
