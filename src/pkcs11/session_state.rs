use std::convert::TryFrom;
use std::os::raw::c_ulong;

use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum SessionState {
    RoPublicSession = 0,
    RoUserFunctions = 1,
    RwPublicSession = 2,
    RwUserFunctions = 3,
    RwSoFunctions = 4,
}

impl TryFrom<c_ulong> for SessionState {
    type Error = ();
    fn try_from(value: c_ulong) -> Result<Self, Self::Error> {
        SessionState::from_u64(value).ok_or(())
    }
}

impl TryFrom<SessionState> for c_ulong {
    type Error = ();
    fn try_from(value: SessionState) -> Result<Self, Self::Error> {
        SessionState::to_u64(&value).ok_or(())
    }
}
