use std::convert::TryFrom;
use std::os::raw::c_ulong;

use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum KeyDerivationFunction {
    Null = 1,
    Sha1Kdf2 = 2,
    Sha1KdfAsn1 = 3,
    Sha1KdfConcatenate = 4,
    Sha224Kdf = 5,
    Sha256Kdf = 6,
    Sha384Kdf = 7,
    Sha512Kdf = 8,
    CpdiversifyKdf = 9,
}

impl TryFrom<c_ulong> for KeyDerivationFunction {
    type Error = ();
    fn try_from(value: c_ulong) -> Result<Self, Self::Error> {
        KeyDerivationFunction::from_u64(value).ok_or(())
    }
}

impl TryFrom<KeyDerivationFunction> for c_ulong {
    type Error = ();
    fn try_from(value: KeyDerivationFunction) -> Result<Self, Self::Error> {
        KeyDerivationFunction::to_u64(&value).ok_or(())
    }
}
