use std::convert::TryFrom;
use std::os::raw::c_ulong;

use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum CertificateCategory {
    Unspecified = 0,
    TokenUser = 1,
    Authority = 2,
    OtherEntity = 3,
}

impl TryFrom<c_ulong> for CertificateCategory {
    type Error = ();
    fn try_from(value: c_ulong) -> Result<Self, Self::Error> {
        CertificateCategory::from_u64(value).ok_or(())
    }
}

impl TryFrom<CertificateCategory> for c_ulong {
    type Error = ();
    fn try_from(value: CertificateCategory) -> Result<Self, Self::Error> {
        CertificateCategory::to_u64(&value).ok_or(())
    }
}
