//! tergent - a cryptoki/PKCS#11 implementation that uses Android keystore as its backend.

mod bridge;
mod key;
mod pkcs11;
mod state;

use std::convert::{TryFrom, TryInto};
use std::os::raw::{c_uchar, c_ulong, c_void};
use std::{slice, str};

use key::Key;
use pkcs11::*;

/// tergent only provides a single slot, and this is its id. It is arbitrarily chosen.
/// To comply with the standard we need to be consistent on which id we use for this slot.
const SLOT_ID: c_ulong = 10;

#[no_mangle]
pub extern "C" fn C_Initialize(_init_args: *mut c_void) -> c_ulong {
    ReturnValue::Ok.try_into().unwrap()
}

#[no_mangle]
pub extern "C" fn C_Finalize(_reserved: *mut c_void) -> c_ulong {
    ReturnValue::Ok.try_into().unwrap()
}

#[no_mangle]
pub extern "C" fn C_GetInfo(info: *mut Info) -> c_ulong {
    let mut info = unsafe { &mut *info };
    info.cryptoki_version.major = CRYPTOKI_VERSION_MAJOR.try_into().unwrap();
    info.cryptoki_version.minor = CRYPTOKI_VERSION_MINOR.try_into().unwrap();
    copy_padded(&mut info.manufacturer_id, "tergent");
    info.flags = 0;
    copy_padded(&mut info.library_description, "tergent");
    info.library_version.major = env!("CARGO_PKG_VERSION_MAJOR").parse().unwrap();
    info.library_version.minor = env!("CARGO_PKG_VERSION_MINOR").parse().unwrap();
    ReturnValue::Ok.try_into().unwrap()
}

#[no_mangle]
pub extern "C" fn C_GetFunctionList(function_list: *mut *mut FunctionList) -> c_ulong {
    unsafe {
        *function_list = &mut FUNCTION_LIST as *mut FunctionList;
    }
    ReturnValue::Ok.try_into().unwrap()
}

#[no_mangle]
pub extern "C" fn C_GetSlotList(
    _token_present: c_uchar,
    slot_list: *mut c_ulong,
    count: *mut c_ulong,
) -> c_ulong {
    // We only have a single slot.
    let count = unsafe { &mut *count };
    if !slot_list.is_null() {
        if *count < 1 {
            return ReturnValue::BufferTooSmall.try_into().unwrap();
        }
        unsafe {
            *slot_list = SLOT_ID;
        }
    }
    *count = 1;
    ReturnValue::Ok.try_into().unwrap()
}

#[no_mangle]
pub extern "C" fn C_GetSlotInfo(slot_id: c_ulong, info: *mut SlotInfo) -> c_ulong {
    if slot_id != SLOT_ID {
        return ReturnValue::SlotIdInvalid.try_into().unwrap();
    }
    let mut slot_info = unsafe { &mut *info };
    copy_padded(&mut slot_info.slot_description, "tergent");
    copy_padded(&mut slot_info.manufacturer_id, "tergent");
    let flags = Flags::TOKEN_PRESENT | Flags::HW_SLOT;
    slot_info.flags = flags.bits().into();
    slot_info.hardware_version.major = 0;
    slot_info.hardware_version.minor = 0;
    slot_info.firmware_version.major = 0;
    slot_info.firmware_version.minor = 0;
    ReturnValue::Ok.try_into().unwrap()
}

#[no_mangle]
pub extern "C" fn C_GetTokenInfo(slot_id: c_ulong, info: *mut TokenInfo) -> c_ulong {
    if slot_id != SLOT_ID {
        return ReturnValue::SlotIdInvalid.try_into().unwrap();
    }

    let session_count = match state::count().try_into() {
        Ok(count) => count,
        Err(_) => {
            return ReturnValue::GeneralError.try_into().unwrap();
        }
    };
    let unavailable_information = UNAVAILABLE_INFORMATION as u64;

    let mut token_info = unsafe { &mut *info };
    copy_padded(&mut token_info.label, "tergent");
    copy_padded(&mut token_info.manufacturer_id, "tergent");
    copy_padded(&mut token_info.model, "tergent");
    copy_padded(&mut token_info.serial_number, "");
    token_info.flags = Flags::TOKEN_INITIALIZED.bits().into();
    // Having simultaneous sessions is not yet supported.
    token_info.max_session_count = 1;
    token_info.session_count = session_count;
    token_info.max_rw_session_count = 1;
    token_info.rw_session_count = session_count;
    token_info.max_pin_len = 0;
    token_info.min_pin_len = 0;
    token_info.total_public_memory = unavailable_information;
    token_info.free_public_memory = unavailable_information;
    token_info.total_private_memory = unavailable_information;
    token_info.free_private_memory = unavailable_information;
    token_info.hardware_version.major = 0;
    token_info.hardware_version.minor = 0;
    token_info.firmware_version.major = 0;
    token_info.firmware_version.minor = 0;
    copy_padded(&mut token_info.utc_time, "");
    ReturnValue::Ok.try_into().unwrap()
}

#[no_mangle]
pub extern "C" fn C_GetMechanismList(
    _slot_id: c_ulong,
    _mechanism_list: *mut c_ulong,
    _count: *mut c_ulong,
) -> c_ulong {
    ReturnValue::FunctionNotSupported.try_into().unwrap()
}

#[no_mangle]
pub extern "C" fn C_GetMechanismInfo(
    _slot_id: c_ulong,
    _type_: c_ulong,
    _info: *mut MechanismInfo,
) -> c_ulong {
    ReturnValue::FunctionNotSupported.try_into().unwrap()
}

#[no_mangle]
pub extern "C" fn C_InitToken(
    _slot_id: c_ulong,
    _pin: *mut c_uchar,
    _pin_len: c_ulong,
    _label: *mut c_uchar,
) -> c_ulong {
    ReturnValue::FunctionNotSupported.try_into().unwrap()
}

#[no_mangle]
pub extern "C" fn C_InitPIN(_session: c_ulong, _pin: *mut c_uchar, _pin_len: c_ulong) -> c_ulong {
    ReturnValue::FunctionNotSupported.try_into().unwrap()
}

#[no_mangle]
pub extern "C" fn C_SetPIN(
    _session: c_ulong,
    _old_pin: *mut c_uchar,
    _old_len: c_ulong,
    _new_pin: *mut c_uchar,
    _new_len: c_ulong,
) -> c_ulong {
    ReturnValue::FunctionNotSupported.try_into().unwrap()
}

#[no_mangle]
pub extern "C" fn C_OpenSession(
    slot_id: c_ulong,
    flags: c_ulong,
    _application: *mut c_void,
    _notify: Option<
        unsafe extern "C" fn(session: c_ulong, event: c_ulong, application: *mut c_void) -> c_ulong,
    >,
    session: *mut c_ulong,
) -> c_ulong {
    // Create a new state object, and return its index as the session id.
    if slot_id != SLOT_ID {
        return ReturnValue::SlotIdInvalid.try_into().unwrap();
    }
    let flags = match flags.try_into() {
        Ok(flags) => flags,
        Err(_) => {
            return ReturnValue::ArgumentsBad.try_into().unwrap();
        }
    };
    let flags = Flags::from_bits_truncate(flags);
    if !flags.contains(Flags::SERIAL_SESSION) {
        return ReturnValue::SessionParallelNotSupported.try_into().unwrap();
    }
    match state::new() {
        Some(index) => {
            unsafe { *session = index }
            ReturnValue::Ok
        }
        None => ReturnValue::GeneralError,
    }
    .try_into()
    .unwrap()
}

#[no_mangle]
pub extern "C" fn C_CloseSession(session: c_ulong) -> c_ulong {
    // Free up the state associated with the given session.
    match state::remove(session) {
        Some(_) => ReturnValue::Ok,
        None => ReturnValue::SessionHandleInvalid,
    }
    .try_into()
    .unwrap()
}

#[no_mangle]
pub extern "C" fn C_CloseAllSessions(_slot_id: c_ulong) -> c_ulong {
    ReturnValue::FunctionNotSupported.try_into().unwrap()
}

#[no_mangle]
pub extern "C" fn C_GetSessionInfo(_session: c_ulong, _info: *mut SessionInfo) -> c_ulong {
    ReturnValue::FunctionNotSupported.try_into().unwrap()
}

#[no_mangle]
pub extern "C" fn C_GetOperationState(
    _session: c_ulong,
    _operation_state: *mut c_uchar,
    _operation_state_len: *mut c_ulong,
) -> c_ulong {
    ReturnValue::FunctionNotSupported.try_into().unwrap()
}

#[no_mangle]
pub extern "C" fn C_SetOperationState(
    _session: c_ulong,
    _operation_state: *mut c_uchar,
    _operation_state_len: c_ulong,
    _encryption_key: c_ulong,
    _authentication_key: c_ulong,
) -> c_ulong {
    ReturnValue::FunctionNotSupported.try_into().unwrap()
}

#[no_mangle]
pub extern "C" fn C_Login(
    _session: c_ulong,
    _user_type: c_ulong,
    _pin: *mut c_uchar,
    _pin_len: c_ulong,
) -> c_ulong {
    ReturnValue::FunctionNotSupported.try_into().unwrap()
}

#[no_mangle]
pub extern "C" fn C_Logout(_session: c_ulong) -> c_ulong {
    ReturnValue::FunctionNotSupported.try_into().unwrap()
}

#[no_mangle]
pub extern "C" fn C_CreateObject(
    _session: c_ulong,
    _template: *mut Attribute,
    _count: c_ulong,
    _object: *mut c_ulong,
) -> c_ulong {
    ReturnValue::FunctionNotSupported.try_into().unwrap()
}

#[no_mangle]
pub extern "C" fn C_CopyObject(
    _session: c_ulong,
    _object: c_ulong,
    _template: *mut Attribute,
    _count: c_ulong,
    _new_object: *mut c_ulong,
) -> c_ulong {
    ReturnValue::FunctionNotSupported.try_into().unwrap()
}

#[no_mangle]
pub extern "C" fn C_DestroyObject(_session: c_ulong, _object: c_ulong) -> c_ulong {
    ReturnValue::FunctionNotSupported.try_into().unwrap()
}

#[no_mangle]
pub extern "C" fn C_GetObjectSize(
    _session: c_ulong,
    _object: c_ulong,
    _size: *mut c_ulong,
) -> c_ulong {
    ReturnValue::FunctionNotSupported.try_into().unwrap()
}

#[no_mangle]
pub extern "C" fn C_GetAttributeValue(
    session: c_ulong,
    object: c_ulong,
    template: *mut Attribute,
    count: c_ulong,
) -> c_ulong {
    // Main function that is used to query the details of a key.
    let state = state::get(session);
    let state = match state {
        Some(state) => state,
        None => {
            return ReturnValue::SessionHandleInvalid.try_into().unwrap();
        }
    };
    let state = state.lock().unwrap();
    let key = match state.get_key(object as usize) {
        Some(key) => key,
        None => {
            return ReturnValue::ObjectHandleInvalid.try_into().unwrap();
        }
    };

    let count = match count.try_into() {
        Ok(count) => count,
        Err(_) => {
            return ReturnValue::ArgumentsBad.try_into().unwrap();
        }
    };
    let templates = unsafe { slice::from_raw_parts_mut(template, count) };
    let mut type_invalid = false;
    for template in templates {
        let attribute_type = match AttributeType::try_from(template.type_) {
            Ok(attribute_type) => attribute_type,
            Err(_) => {
                template.set_value_single(UNAVAILABLE_INFORMATION as u64);
                type_invalid = true;
                continue;
            }
        };
        match attribute_type {
            AttributeType::KeyType => {
                let key_type = match key {
                    Key::Rsa(_) => KeyType::Rsa,
                    Key::Ec(_) => KeyType::Ec,
                }
                .try_into()
                .unwrap();
                template.set_value_single(key_type);
            }
            AttributeType::Label | AttributeType::Id => {
                let label = key.label().as_bytes();
                template.set_value(&label);
            }
            AttributeType::Modulus => {
                if let Key::Rsa(key) = key {
                    template.set_value(&key.modulus());
                } else {
                    template.set_value_single(UNAVAILABLE_INFORMATION as u64);
                    type_invalid = true;
                }
            }
            AttributeType::PublicExponent => {
                if let Key::Rsa(key) = key {
                    template.set_value(&key.exponent());
                } else {
                    template.set_value_single(UNAVAILABLE_INFORMATION as u64);
                    type_invalid = true;
                }
            }
            AttributeType::EcPoint => {
                if let Key::Ec(key) = key {
                    let point = key.point_as_asn1();
                    if let Ok(point) = point {
                        template.set_value(&point);
                        continue;
                    }
                }
                template.set_value_single(UNAVAILABLE_INFORMATION as u64);
                type_invalid = true;
            }
            AttributeType::EcParams => {
                if let Key::Ec(key) = key {
                    let params = key.params_as_asn1();
                    if let Ok(params) = params {
                        template.set_value(&params);
                        continue;
                    }
                }
                template.set_value_single(UNAVAILABLE_INFORMATION as u64);
                type_invalid = true;
            }
            AttributeType::AlwaysAuthenticate => {
                template.set_value_single(0);
            }
            _ => {
                template.set_value_single(UNAVAILABLE_INFORMATION as u64);
                type_invalid = true;
            }
        };
    }
    if type_invalid {
        ReturnValue::AttributeTypeInvalid
    } else {
        ReturnValue::Ok
    }
    .try_into()
    .unwrap()
}

#[no_mangle]
pub extern "C" fn C_SetAttributeValue(
    _session: c_ulong,
    _object: c_ulong,
    _template: *mut Attribute,
    _count: c_ulong,
) -> c_ulong {
    ReturnValue::FunctionNotSupported.try_into().unwrap()
}

#[no_mangle]
pub extern "C" fn C_FindObjectsInit(
    session: c_ulong,
    template: *mut Attribute,
    count: c_ulong,
) -> c_ulong {
    // Initialize a search operation.
    let mut find_keys = false;
    let mut find_id = None;

    let count = match count.try_into() {
        Ok(count) => count,
        Err(_) => {
            return ReturnValue::ArgumentsBad.try_into().unwrap();
        }
    };
    let templates = unsafe { slice::from_raw_parts_mut(template, count) };
    for template in templates {
        match AttributeType::try_from(template.type_) {
            Ok(AttributeType::Class) => {
                // We only support searching for public/private keys for now.
                let value = template.value as *mut u64;
                let class = unsafe { *value };
                if let Ok(ObjectClass::PublicKey) | Ok(ObjectClass::PrivateKey) = class.try_into() {
                    find_keys = true;
                }
            }
            Ok(AttributeType::Id) => {
                // The application wants to search for a specific key with the given id/label.
                let value = template.value as *mut u8;
                let length = template.value_len.try_into().unwrap();
                let id = unsafe { slice::from_raw_parts_mut(value, length) };
                if let Ok(id) = str::from_utf8(id) {
                    find_id = Some(String::from(id));
                } else {
                    return ReturnValue::AttributeValueInvalid.try_into().unwrap();
                }
            }
            Ok(AttributeType::Sign) => {
                // All the keys can be used for signing, so ignore this attribute.
            }
            _ => {
                // Unknown attribute, abort the operation.
                return ReturnValue::AttributeTypeInvalid.try_into().unwrap();
            }
        }
    }
    let state = state::get(session);
    let state = match state {
        Some(state) => state,
        None => {
            return ReturnValue::SessionHandleInvalid.try_into().unwrap();
        }
    };
    let mut state = state.lock().unwrap();
    state.find_init(find_keys, find_id);

    ReturnValue::Ok.try_into().unwrap()
}

#[no_mangle]
pub extern "C" fn C_FindObjects(
    session: c_ulong,
    object: *mut c_ulong,
    max_object_count: c_ulong,
    object_count: *mut c_ulong,
) -> c_ulong {
    // Continue a search operation.
    let object_count = unsafe { &mut *object_count };
    *object_count = 0;

    let state = state::get(session);
    let state = match state {
        Some(state) => state,
        None => {
            return ReturnValue::SessionHandleInvalid.try_into().unwrap();
        }
    };
    let mut state = state.lock().unwrap();

    let max_object_count = match max_object_count.try_into() {
        Ok(max_object_count) => max_object_count,
        Err(_) => {
            return ReturnValue::ArgumentsBad.try_into().unwrap();
        }
    };
    let objects = unsafe { slice::from_raw_parts_mut(object, max_object_count) };
    for object in objects {
        let key_index = state.find_next();
        match key_index {
            Some(index) => {
                *object = index.try_into().unwrap();
                *object_count += 1;
            }
            None => break,
        }
    }
    ReturnValue::Ok.try_into().unwrap()
}

#[no_mangle]
pub extern "C" fn C_FindObjectsFinal(_session: c_ulong) -> c_ulong {
    ReturnValue::Ok.try_into().unwrap()
}

#[no_mangle]
pub extern "C" fn C_EncryptInit(
    _session: c_ulong,
    _mechanism: *mut Mechanism,
    _key: c_ulong,
) -> c_ulong {
    ReturnValue::FunctionNotSupported.try_into().unwrap()
}

#[no_mangle]
pub extern "C" fn C_Encrypt(
    _session: c_ulong,
    _data: *mut c_uchar,
    _data_len: c_ulong,
    _encrypted_data: *mut c_uchar,
    _encrypted_data_len: *mut c_ulong,
) -> c_ulong {
    ReturnValue::FunctionNotSupported.try_into().unwrap()
}

#[no_mangle]
pub extern "C" fn C_EncryptUpdate(
    _session: c_ulong,
    _part: *mut c_uchar,
    _part_len: c_ulong,
    _encrypted_part: *mut c_uchar,
    _encrypted_part_len: *mut c_ulong,
) -> c_ulong {
    ReturnValue::FunctionNotSupported.try_into().unwrap()
}

#[no_mangle]
pub extern "C" fn C_EncryptFinal(
    _session: c_ulong,
    _last_encrypted_part: *mut c_uchar,
    _last_encrypted_part_len: *mut c_ulong,
) -> c_ulong {
    ReturnValue::FunctionNotSupported.try_into().unwrap()
}

#[no_mangle]
pub extern "C" fn C_DecryptInit(
    _session: c_ulong,
    _mechanism: *mut Mechanism,
    _key: c_ulong,
) -> c_ulong {
    ReturnValue::FunctionNotSupported.try_into().unwrap()
}

#[no_mangle]
pub extern "C" fn C_Decrypt(
    _session: c_ulong,
    _encrypted_data: *mut c_uchar,
    _encrypted_data_len: c_ulong,
    _data: *mut c_uchar,
    _data_len: *mut c_ulong,
) -> c_ulong {
    ReturnValue::FunctionNotSupported.try_into().unwrap()
}

#[no_mangle]
pub extern "C" fn C_DecryptUpdate(
    _session: c_ulong,
    _encrypted_part: *mut c_uchar,
    _encrypted_part_len: c_ulong,
    _part: *mut c_uchar,
    _part_len: *mut c_ulong,
) -> c_ulong {
    ReturnValue::FunctionNotSupported.try_into().unwrap()
}

#[no_mangle]
pub extern "C" fn C_DecryptFinal(
    _session: c_ulong,
    _last_part: *mut c_uchar,
    _last_part_len: *mut c_ulong,
) -> c_ulong {
    ReturnValue::FunctionNotSupported.try_into().unwrap()
}

#[no_mangle]
pub extern "C" fn C_DigestInit(_session: c_ulong, _mechanism: *mut Mechanism) -> c_ulong {
    ReturnValue::FunctionNotSupported.try_into().unwrap()
}

#[no_mangle]
pub extern "C" fn C_Digest(
    _session: c_ulong,
    _data: *mut c_uchar,
    _data_len: c_ulong,
    _digest: *mut c_uchar,
    _digest_len: *mut c_ulong,
) -> c_ulong {
    ReturnValue::FunctionNotSupported.try_into().unwrap()
}

#[no_mangle]
pub extern "C" fn C_DigestUpdate(
    _session: c_ulong,
    _part: *mut c_uchar,
    _part_len: c_ulong,
) -> c_ulong {
    ReturnValue::FunctionNotSupported.try_into().unwrap()
}

#[no_mangle]
pub extern "C" fn C_DigestKey(_session: c_ulong, _key: c_ulong) -> c_ulong {
    ReturnValue::FunctionNotSupported.try_into().unwrap()
}

#[no_mangle]
pub extern "C" fn C_DigestFinal(
    _session: c_ulong,
    _digest: *mut c_uchar,
    _digest_len: *mut c_ulong,
) -> c_ulong {
    ReturnValue::FunctionNotSupported.try_into().unwrap()
}

#[no_mangle]
pub extern "C" fn C_SignInit(session: c_ulong, mechanism: *mut Mechanism, key: c_ulong) -> c_ulong {
    // Initialize a sign operation.
    let state = state::get(session);
    let state = match state {
        Some(state) => state,
        None => {
            return ReturnValue::SessionHandleInvalid.try_into().unwrap();
        }
    };
    let mut state = state.lock().unwrap();

    let index = key.try_into().unwrap();
    let key = state.get_key(index);
    let key = match key {
        Some(key) => key,
        None => {
            return ReturnValue::KeyHandleInvalid.try_into().unwrap();
        }
    };
    let mechanism = unsafe { &*mechanism };
    // Android keystore supports more mechanisms but these two are the bare minimum required
    // for ssh to function.
    match MechanismType::try_from(mechanism.mechanism) {
        Ok(MechanismType::Ecdsa) => {
            if let key::Key::Ec(_) = key {
                state.sign_init(index);
            } else {
                return ReturnValue::KeyTypeInconsistent.try_into().unwrap();
            }
        }
        Ok(MechanismType::RsaPkcs) => {
            if let key::Key::Rsa(_) = key {
                state.sign_init(index);
            } else {
                return ReturnValue::KeyTypeInconsistent.try_into().unwrap();
            }
        }
        _ => {
            return ReturnValue::MechanismInvalid.try_into().unwrap();
        }
    }
    ReturnValue::Ok.try_into().unwrap()
}

#[no_mangle]
pub extern "C" fn C_Sign(
    session: c_ulong,
    data: *mut c_uchar,
    data_len: c_ulong,
    signature: *mut c_uchar,
    signature_len: *mut c_ulong,
) -> c_ulong {
    // Calculate the signature.
    let signature_out = {
        let data = unsafe { slice::from_raw_parts_mut(data, data_len.try_into().unwrap()) };

        let state = state::get(session);
        let state = match state {
            Some(state) => state,
            None => {
                return ReturnValue::SessionHandleInvalid.try_into().unwrap();
            }
        };
        let state = state.lock().unwrap();

        let key = state.get_sign_key().unwrap();
        if let Some(signature) = key.sign(data) {
            signature
        } else {
            return ReturnValue::GeneralError.try_into().unwrap();
        }
    };
    // Size required to represent the signature output.
    let signature_out_len = signature_out.len().try_into().unwrap();
    // Size provided to us to place the signature output.
    let signature_len = unsafe { &mut *signature_len };

    // No buffer is given, just tell the application how much space we need.
    if signature.is_null() {
        *signature_len = signature_out_len;
        return ReturnValue::Ok.try_into().unwrap();
    }
    // Buffer is too small.
    if *signature_len < signature_out.len().try_into().unwrap() {
        *signature_len = signature_out_len;
        return ReturnValue::BufferTooSmall.try_into().unwrap();
    }

    let signature = unsafe { slice::from_raw_parts_mut(signature, signature_out.len()) };
    signature.copy_from_slice(&signature_out);
    *signature_len = signature_out_len;
    ReturnValue::Ok.try_into().unwrap()
}

#[no_mangle]
pub extern "C" fn C_SignUpdate(
    _session: c_ulong,
    _part: *mut c_uchar,
    _part_len: c_ulong,
) -> c_ulong {
    ReturnValue::FunctionNotSupported.try_into().unwrap()
}

#[no_mangle]
pub extern "C" fn C_SignFinal(
    _session: c_ulong,
    _signature: *mut c_uchar,
    _signature_len: *mut c_ulong,
) -> c_ulong {
    ReturnValue::Ok.try_into().unwrap()
}

#[no_mangle]
pub extern "C" fn C_SignRecoverInit(
    _session: c_ulong,
    _mechanism: *mut Mechanism,
    _key: c_ulong,
) -> c_ulong {
    ReturnValue::FunctionNotSupported.try_into().unwrap()
}

#[no_mangle]
pub extern "C" fn C_SignRecover(
    _session: c_ulong,
    _data: *mut c_uchar,
    _data_len: c_ulong,
    _signature: *mut c_uchar,
    _signature_len: *mut c_ulong,
) -> c_ulong {
    ReturnValue::FunctionNotSupported.try_into().unwrap()
}

#[no_mangle]
pub extern "C" fn C_VerifyInit(
    _session: c_ulong,
    _mechanism: *mut Mechanism,
    _key: c_ulong,
) -> c_ulong {
    ReturnValue::FunctionNotSupported.try_into().unwrap()
}

#[no_mangle]
pub extern "C" fn C_Verify(
    _session: c_ulong,
    _data: *mut c_uchar,
    _data_len: c_ulong,
    _signature: *mut c_uchar,
    _signature_len: c_ulong,
) -> c_ulong {
    ReturnValue::FunctionNotSupported.try_into().unwrap()
}

#[no_mangle]
pub extern "C" fn C_VerifyUpdate(
    _session: c_ulong,
    _part: *mut c_uchar,
    _part_len: c_ulong,
) -> c_ulong {
    ReturnValue::FunctionNotSupported.try_into().unwrap()
}

#[no_mangle]
pub extern "C" fn C_VerifyFinal(
    _session: c_ulong,
    _signature: *mut c_uchar,
    _signature_len: c_ulong,
) -> c_ulong {
    ReturnValue::FunctionNotSupported.try_into().unwrap()
}

#[no_mangle]
pub extern "C" fn C_VerifyRecoverInit(
    _session: c_ulong,
    _mechanism: *mut Mechanism,
    _key: c_ulong,
) -> c_ulong {
    ReturnValue::FunctionNotSupported.try_into().unwrap()
}

#[no_mangle]
pub extern "C" fn C_VerifyRecover(
    _session: c_ulong,
    _signature: *mut c_uchar,
    _signature_len: c_ulong,
    _data: *mut c_uchar,
    _data_len: *mut c_ulong,
) -> c_ulong {
    ReturnValue::FunctionNotSupported.try_into().unwrap()
}

#[no_mangle]
pub extern "C" fn C_DigestEncryptUpdate(
    _session: c_ulong,
    _part: *mut c_uchar,
    _part_len: c_ulong,
    _encrypted_part: *mut c_uchar,
    _encrypted_part_len: *mut c_ulong,
) -> c_ulong {
    ReturnValue::FunctionNotSupported.try_into().unwrap()
}

#[no_mangle]
pub extern "C" fn C_DecryptDigestUpdate(
    _session: c_ulong,
    _encrypted_part: *mut c_uchar,
    _encrypted_part_len: c_ulong,
    _part: *mut c_uchar,
    _part_len: *mut c_ulong,
) -> c_ulong {
    ReturnValue::FunctionNotSupported.try_into().unwrap()
}

#[no_mangle]
pub extern "C" fn C_SignEncryptUpdate(
    _session: c_ulong,
    _part: *mut c_uchar,
    _part_len: c_ulong,
    _encrypted_part: *mut c_uchar,
    _encrypted_part_len: *mut c_ulong,
) -> c_ulong {
    ReturnValue::FunctionNotSupported.try_into().unwrap()
}

#[no_mangle]
pub extern "C" fn C_DecryptVerifyUpdate(
    _session: c_ulong,
    _encrypted_part: *mut c_uchar,
    _encrypted_part_len: c_ulong,
    _part: *mut c_uchar,
    _part_len: *mut c_ulong,
) -> c_ulong {
    ReturnValue::FunctionNotSupported.try_into().unwrap()
}

#[no_mangle]
pub extern "C" fn C_GenerateKey(
    _session: c_ulong,
    _mechanism: *mut Mechanism,
    _template: *mut Attribute,
    _count: c_ulong,
    _key: *mut c_ulong,
) -> c_ulong {
    ReturnValue::FunctionNotSupported.try_into().unwrap()
}

#[no_mangle]
pub extern "C" fn C_GenerateKeyPair(
    _session: c_ulong,
    _mechanism: *mut Mechanism,
    _public_key_template: *mut Attribute,
    _public_key_attribute_count: c_ulong,
    _private_key_template: *mut Attribute,
    _private_key_attribute_count: c_ulong,
    _public_key: *mut c_ulong,
    _private_key: *mut c_ulong,
) -> c_ulong {
    ReturnValue::FunctionNotSupported.try_into().unwrap()
}

#[no_mangle]
pub extern "C" fn C_WrapKey(
    _session: c_ulong,
    _mechanism: *mut Mechanism,
    _wrapping_key: c_ulong,
    _key: c_ulong,
    _wrapped_key: *mut c_uchar,
    _wrapped_key_len: *mut c_ulong,
) -> c_ulong {
    ReturnValue::FunctionNotSupported.try_into().unwrap()
}

#[no_mangle]
pub extern "C" fn C_UnwrapKey(
    _session: c_ulong,
    _mechanism: *mut Mechanism,
    _unwrapping_key: c_ulong,
    _wrapped_key: *mut c_uchar,
    _wrapped_key_len: c_ulong,
    _template: *mut Attribute,
    _attribute_count: c_ulong,
    _key: *mut c_ulong,
) -> c_ulong {
    ReturnValue::FunctionNotSupported.try_into().unwrap()
}

#[no_mangle]
pub extern "C" fn C_DeriveKey(
    _session: c_ulong,
    _mechanism: *mut Mechanism,
    _base_key: c_ulong,
    _template: *mut Attribute,
    _attribute_count: c_ulong,
    _key: *mut c_ulong,
) -> c_ulong {
    ReturnValue::FunctionNotSupported.try_into().unwrap()
}

#[no_mangle]
pub extern "C" fn C_SeedRandom(
    _session: c_ulong,
    _seed: *mut c_uchar,
    _seed_len: c_ulong,
) -> c_ulong {
    ReturnValue::FunctionNotSupported.try_into().unwrap()
}

#[no_mangle]
pub extern "C" fn C_GenerateRandom(
    _session: c_ulong,
    _random_data: *mut c_uchar,
    _random_len: c_ulong,
) -> c_ulong {
    ReturnValue::FunctionNotSupported.try_into().unwrap()
}

#[no_mangle]
pub extern "C" fn C_GetFunctionStatus(_session: c_ulong) -> c_ulong {
    ReturnValue::FunctionNotSupported.try_into().unwrap()
}

#[no_mangle]
pub extern "C" fn C_CancelFunction(_session: c_ulong) -> c_ulong {
    ReturnValue::FunctionNotSupported.try_into().unwrap()
}

#[no_mangle]
pub extern "C" fn C_WaitForSlotEvent(
    _flags: c_ulong,
    _slot: *mut c_ulong,
    _rserved: *mut c_void,
) -> c_ulong {
    ReturnValue::FunctionNotSupported.try_into().unwrap()
}

static mut FUNCTION_LIST: FunctionList = FunctionList {
    version: Version {
        major: 2,
        minor: 40,
    },
    initialize: Some(C_Initialize),
    finalize: Some(C_Finalize),
    get_info: Some(C_GetInfo),
    get_function_list: Some(C_GetFunctionList),
    get_slot_list: Some(C_GetSlotList),
    get_slot_info: Some(C_GetSlotInfo),
    get_token_info: Some(C_GetTokenInfo),
    get_mechanism_list: Some(C_GetMechanismList),
    get_mechanism_info: Some(C_GetMechanismInfo),
    init_token: Some(C_InitToken),
    init_pin: Some(C_InitPIN),
    set_pin: Some(C_SetPIN),
    open_session: Some(C_OpenSession),
    close_session: Some(C_CloseSession),
    close_all_sessions: Some(C_CloseAllSessions),
    get_session_info: Some(C_GetSessionInfo),
    get_operation_state: Some(C_GetOperationState),
    set_operation_state: Some(C_SetOperationState),
    login: Some(C_Login),
    logout: Some(C_Logout),
    create_object: Some(C_CreateObject),
    copy_object: Some(C_CopyObject),
    destroy_object: Some(C_DestroyObject),
    get_object_size: Some(C_GetObjectSize),
    get_attribute_value: Some(C_GetAttributeValue),
    set_attribute_value: Some(C_SetAttributeValue),
    find_objects_init: Some(C_FindObjectsInit),
    find_objects: Some(C_FindObjects),
    find_objects_final: Some(C_FindObjectsFinal),
    encrypt_init: Some(C_EncryptInit),
    encrypt: Some(C_Encrypt),
    encrypt_update: Some(C_EncryptUpdate),
    encrypt_final: Some(C_EncryptFinal),
    decrypt_init: Some(C_DecryptInit),
    decrypt: Some(C_Decrypt),
    decrypt_update: Some(C_DecryptUpdate),
    decrypt_final: Some(C_DecryptFinal),
    digest_init: Some(C_DigestInit),
    digest: Some(C_Digest),
    digest_update: Some(C_DigestUpdate),
    digest_key: Some(C_DigestKey),
    digest_final: Some(C_DigestFinal),
    sign_init: Some(C_SignInit),
    sign: Some(C_Sign),
    sign_update: Some(C_SignUpdate),
    sign_final: Some(C_SignFinal),
    sign_recover_init: Some(C_SignRecoverInit),
    sign_recover: Some(C_SignRecover),
    verify_init: Some(C_VerifyInit),
    verify: Some(C_Verify),
    verify_update: Some(C_VerifyUpdate),
    verify_final: Some(C_VerifyFinal),
    verify_recover_init: Some(C_VerifyRecoverInit),
    verify_recover: Some(C_VerifyRecover),
    digest_encrypt_update: Some(C_DigestEncryptUpdate),
    decrypt_digest_update: Some(C_DecryptDigestUpdate),
    sign_encrypt_update: Some(C_SignEncryptUpdate),
    decrypt_verify_update: Some(C_DecryptVerifyUpdate),
    generate_key: Some(C_GenerateKey),
    generate_key_pair: Some(C_GenerateKeyPair),
    wrap_key: Some(C_WrapKey),
    unwrap_key: Some(C_UnwrapKey),
    derive_key: Some(C_DeriveKey),
    seed_random: Some(C_SeedRandom),
    generate_random: Some(C_GenerateRandom),
    get_function_status: Some(C_GetFunctionStatus),
    cancel_function: Some(C_CancelFunction),
    wait_for_slot_event: Some(C_WaitForSlotEvent),
};
