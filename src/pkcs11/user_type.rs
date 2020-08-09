use std::convert::TryFrom;
use std::os::raw::c_ulong;

use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum UserType {
    So = 0,
    User = 1,
    ContextSpecific = 2,
}

impl TryFrom<c_ulong> for UserType {
    type Error = ();
    fn try_from(value: c_ulong) -> Result<Self, Self::Error> {
        UserType::from_u64(value).ok_or(())
    }
}

impl TryFrom<UserType> for c_ulong {
    type Error = ();
    fn try_from(value: UserType) -> Result<Self, Self::Error> {
        UserType::to_u64(&value).ok_or(())
    }
}
