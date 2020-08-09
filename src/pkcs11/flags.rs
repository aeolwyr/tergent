use bitflags::bitflags;

bitflags! {
    pub struct Flags: u32 {
        const TOKEN_PRESENT                 = 0x00000001;
        const REMOVABLE_DEVICE              = 0x00000002;
        const HW_SLOT                       = 0x00000004;
        const RNG                           = 0x00000001;
        const WRITE_PROTECTED               = 0x00000002;
        const LOGIN_REQUIRED                = 0x00000004;
        const USER_PIN_INITIALIZED          = 0x00000008;
        const RESTORE_KEY_NOT_NEEDED        = 0x00000020;
        const CLOCK_ON_TOKEN                = 0x00000040;
        const PROTECTED_AUTHENTICATION_PATH = 0x00000100;
        const DUAL_CRYPTO_OPERATIONS        = 0x00000200;
        const TOKEN_INITIALIZED             = 0x00000400;
        const SECONDARY_AUTHENTICATION      = 0x00000800;
        const USER_PIN_COUNT_LOW            = 0x00010000;
        const USER_PIN_FINAL_TRY            = 0x00020000;
        const USER_PIN_LOCKED               = 0x00040000;
        const USER_PIN_TO_BE_CHANGED        = 0x00080000;
        const SO_PIN_COUNT_LOW              = 0x00100000;
        const SO_PIN_FINAL_TRY              = 0x00200000;
        const SO_PIN_LOCKED                 = 0x00400000;
        const SO_PIN_TO_BE_CHANGED          = 0x00800000;
        const ERROR_STATE                   = 0x01000000;

        const RW_SESSION     = 0x00000002;
        const SERIAL_SESSION = 0x00000004;

        const ARRAY_ATTRIBUTE = 0x40000000;

        const HW                 = 0x00000001;
        const ENCRYPT            = 0x00000100;
        const DECRYPT            = 0x00000200;
        const DIGEST             = 0x00000400;
        const SIGN               = 0x00000800;
        const SIGN_RECOVER       = 0x00001000;
        const VERIFY             = 0x00002000;
        const VERIFY_RECOVER     = 0x00004000;
        const GENERATE           = 0x00008000;
        const GENERATE_KEY_PAIR  = 0x00010000;
        const WRAP               = 0x00020000;
        const UNWRAP             = 0x00040000;
        const DERIVE             = 0x00080000;
        const EC_F_P             = 0x00100000;
        const EC_F_2M            = 0x00200000;
        const EC_ECPARAMETERS    = 0x00400000;
        const EC_NAMEDCURVE      = 0x00800000;
        const EC_UNCOMPRESS      = 0x01000000;
        const EC_COMPRESS        = 0x02000000;
        const EXTENSION          = 0x80000000;

        const LIBRARY_CANT_CREATE_OS_THREADS = 0x00000001;
        const OS_LOCKING_OK                  = 0x00000002;
        const DONT_BLOCK                     = 1;

        const NEXT_OTP          = 0x00000001;
        const EXCLUDE_TIME      = 0x00000002;
        const EXCLUDE_COUNTER   = 0x00000004;
        const EXCLUDE_CHALLENGE = 0x00000008;
        const EXCLUDE_PIN       = 0x00000010;
        const USER_FRIENDLY_OTP = 0x00000020;
    }
}
