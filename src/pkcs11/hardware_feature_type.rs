use std::convert::TryFrom;
use std::os::raw::c_ulong;

use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

#[repr(u64)]
#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum HardwareFeatureType {
    MonotonicCounter = 0x01,
    Clock = 0x02,
    UserInterface = 0x03,
    VendorDefined = 0x80000000,
}

impl TryFrom<c_ulong> for HardwareFeatureType {
    type Error = ();
    fn try_from(value: c_ulong) -> Result<Self, Self::Error> {
        HardwareFeatureType::from_u64(value).ok_or(())
    }
}

impl TryFrom<HardwareFeatureType> for c_ulong {
    type Error = ();
    fn try_from(value: HardwareFeatureType) -> Result<Self, Self::Error> {
        HardwareFeatureType::to_u64(&value).ok_or(())
    }
}
