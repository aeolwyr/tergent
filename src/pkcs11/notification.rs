use std::convert::TryFrom;
use std::os::raw::c_ulong;

use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum Notification {
    Surrender = 0,
    OtpChanged = 1,
}

#[cfg(target_pointer_width = "32")]
impl TryFrom<c_ulong> for Notification {
    type Error = ();
    fn try_from(value: c_ulong) -> Result<Self, Self::Error> {
        Notification::from_u32(value).ok_or(())
    }
}
#[cfg(target_pointer_width = "32")]
impl TryFrom<Notification> for c_ulong {
    type Error = ();
    fn try_from(value: Notification) -> Result<Self, Self::Error> {
        Notification::to_u32(&value).ok_or(())
    }
}

#[cfg(target_pointer_width = "64")]
impl TryFrom<c_ulong> for Notification {
    type Error = ();
    fn try_from(value: c_ulong) -> Result<Self, Self::Error> {
        Notification::from_u64(value).ok_or(())
    }
}
#[cfg(target_pointer_width = "64")]
impl TryFrom<Notification> for c_ulong {
    type Error = ();
    fn try_from(value: Notification) -> Result<Self, Self::Error> {
        Notification::to_u64(&value).ok_or(())
    }
}
