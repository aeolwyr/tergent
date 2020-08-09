use std::convert::TryFrom;
use std::os::raw::c_ulong;

use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum ReturnValue {
    Ok = 0x0000,
    Cancel = 0x0001,
    HostMemory = 0x0002,
    SlotIdInvalid = 0x0003,
    GeneralError = 0x0005,
    FunctionFailed = 0x0006,
    ArgumentsBad = 0x0007,
    NoEvent = 0x0008,
    NeedToCreateThreads = 0x0009,
    CantLock = 0x000a,
    AttributeReadOnly = 0x0010,
    AttributeSensitive = 0x0011,
    AttributeTypeInvalid = 0x0012,
    AttributeValueInvalid = 0x0013,
    ActionProhibited = 0x001b,
    DataInvalid = 0x0020,
    DataLenRange = 0x0021,
    DeviceError = 0x0030,
    DeviceMemory = 0x0031,
    DeviceRemoved = 0x0032,
    EncryptedDataInvalid = 0x0040,
    EncryptedDataLenRange = 0x0041,
    FunctionCanceled = 0x0050,
    FunctionNotParallel = 0x0051,
    FunctionNotSupported = 0x0054,
    KeyHandleInvalid = 0x0060,
    KeySizeRange = 0x0062,
    KeyTypeInconsistent = 0x0063,
    KeyNotNeeded = 0x0064,
    KeyChanged = 0x0065,
    KeyNeeded = 0x0066,
    KeyIndigestible = 0x0067,
    KeyFunctionNotPermitted = 0x0068,
    KeyNotWrappable = 0x0069,
    KeyUnextractable = 0x006a,
    MechanismInvalid = 0x0070,
    MechanismParamInvalid = 0x0071,
    ObjectHandleInvalid = 0x0082,
    OperationActive = 0x0090,
    OperationNotInitialized = 0x0091,
    PinIncorrect = 0x00a0,
    PinInvalid = 0x00a1,
    PinLenRange = 0x00a2,
    PinExpired = 0x00a3,
    PinLocked = 0x00a4,
    SessionClosed = 0x00b0,
    SessionCount = 0x00b1,
    SessionHandleInvalid = 0x00b3,
    SessionParallelNotSupported = 0x00b4,
    SessionReadOnly = 0x00b5,
    SessionExists = 0x00b6,
    SessionReadOnlyExists = 0x00b7,
    SessionReadWriteSoExists = 0x00b8,
    SignatureInvalid = 0x00c0,
    SignatureLenRange = 0x00c1,
    TemplateIncomplete = 0x00d0,
    TemplateInconsistent = 0x00d1,
    TokenNotPresent = 0x00e0,
    TokenNotRecognized = 0x00e1,
    TokenWriteProtected = 0x00e2,
    UnwrappingKeyHandleInvalid = 0x00f0,
    UnwrappingKeySizeRange = 0x00f1,
    UnwrappingKeyTypeInconsistent = 0x00f2,
    UserAlreadyLoggedIn = 0x0100,
    UserNotLoggedIn = 0x0101,
    UserPinNotInitialized = 0x0102,
    UserTypeInvalid = 0x0103,
    UserAnotherAlreadyLoggedIn = 0x0104,
    UserTooManyTypes = 0x0105,
    WrappedKeyInvalid = 0x0110,
    WrappedKeyLenRange = 0x0112,
    WrappingKeyHandleInvalid = 0x0113,
    WrappingKeySizeRange = 0x0114,
    WrappingKeyTypeInconsistent = 0x0115,
    RandomSeedNotSupported = 0x0120,
    RandomNoRng = 0x0121,
    DomainParamsInvalid = 0x0130,
    CurveNotSupported = 0x0140,
    BufferTooSmall = 0x0150,
    SavedStateInvalid = 0x0160,
    InformationSensitive = 0x0170,
    StateUnsaveable = 0x0180,
    CryptokiNotInitialized = 0x0190,
    CryptokiAlreadyInitialized = 0x0191,
    MutexBad = 0x01a0,
    MutexNotLocked = 0x01a1,
    NewPinMode = 0x01b0,
    NextOtp = 0x01b1,
    ExceededMaxIterations = 0x01b5,
    FipsSelfTestFailed = 0x01b6,
    LibraryLoadFailed = 0x01b7,
    PinTooWeak = 0x01b8,
    PublicKeyInvalid = 0x01b9,
    FunctionRejected = 0x0200,
    VendorDefined = 0x80000000,
}

impl TryFrom<c_ulong> for ReturnValue {
    type Error = ();
    fn try_from(value: c_ulong) -> Result<Self, Self::Error> {
        ReturnValue::from_u64(value).ok_or(())
    }
}

impl TryFrom<ReturnValue> for c_ulong {
    type Error = ();
    fn try_from(value: ReturnValue) -> Result<Self, Self::Error> {
        ReturnValue::to_u64(&value).ok_or(())
    }
}
