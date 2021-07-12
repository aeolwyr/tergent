use std::convert::TryFrom;
use std::os::raw::c_ulong;

use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

#[repr(u64)]
#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum KeyType {
    Rsa = 0x0000,
    Dsa = 0x0001,
    Dh = 0x0002,
    Ec = 0x0003,
    X942Dh = 0x0004,
    Kea = 0x0005,
    GenericSecret = 0x0010,
    Rc2 = 0x0011,
    Rc4 = 0x0012,
    Des = 0x0013,
    Des2 = 0x0014,
    Des3 = 0x0015,
    Cast = 0x0016,
    Cast3 = 0x0017,
    Cast128 = 0x0018,
    Rc5 = 0x0019,
    Idea = 0x001a,
    Skipjack = 0x001b,
    Baton = 0x001c,
    Juniper = 0x001d,
    Cdmf = 0x001e,
    Aes = 0x001f,
    Blowfish = 0x0020,
    Twofish = 0x0021,
    Securid = 0x0022,
    Hotp = 0x0023,
    Acti = 0x0024,
    Camellia = 0x0025,
    Aria = 0x0026,
    Md5Hmac = 0x0027,
    Sha1Hmac = 0x0028,
    Ripemd128Hmac = 0x0029,
    Ripemd160Hmac = 0x002a,
    Sha256Hmac = 0x002b,
    Sha384Hmac = 0x002c,
    Sha512Hmac = 0x002d,
    Sha224Hmac = 0x002e,
    Seed = 0x002f,
    Gostr3410 = 0x0030,
    Gostr3411 = 0x0031,
    Gost28147 = 0x0032,
    VendorDefined = 0x80000000,
}

impl TryFrom<c_ulong> for KeyType {
    type Error = ();
    fn try_from(value: c_ulong) -> Result<Self, Self::Error> {
        KeyType::from_u64(value).ok_or(())
    }
}

impl TryFrom<KeyType> for c_ulong {
    type Error = ();
    fn try_from(value: KeyType) -> Result<Self, Self::Error> {
        KeyType::to_u64(&value).ok_or(())
    }
}
