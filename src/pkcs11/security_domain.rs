use std::convert::TryFrom;
use std::os::raw::c_ulong;

use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum SecurityDomain {
    Unspecified = 0,
    Manufacturer = 1,
    Operator = 2,
    ThirdParty = 3,
}

impl TryFrom<c_ulong> for SecurityDomain {
    type Error = ();
    fn try_from(value: c_ulong) -> Result<Self, Self::Error> {
        SecurityDomain::from_u64(value).ok_or(())
    }
}

impl TryFrom<SecurityDomain> for c_ulong {
    type Error = ();
    fn try_from(value: SecurityDomain) -> Result<Self, Self::Error> {
        SecurityDomain::to_u64(&value).ok_or(())
    }
}
