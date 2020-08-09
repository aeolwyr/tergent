use std::os::raw::{c_uchar, c_ulong, c_void};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Version {
    pub major: c_uchar,
    pub minor: c_uchar,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Info {
    pub cryptoki_version: Version,
    pub manufacturer_id: [c_uchar; 32usize],
    pub flags: c_ulong,
    pub library_description: [c_uchar; 32usize],
    pub library_version: Version,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SlotInfo {
    pub slot_description: [c_uchar; 64usize],
    pub manufacturer_id: [c_uchar; 32usize],
    pub flags: c_ulong,
    pub hardware_version: Version,
    pub firmware_version: Version,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TokenInfo {
    pub label: [c_uchar; 32usize],
    pub manufacturer_id: [c_uchar; 32usize],
    pub model: [c_uchar; 16usize],
    pub serial_number: [c_uchar; 16usize],
    pub flags: c_ulong,
    pub max_session_count: c_ulong,
    pub session_count: c_ulong,
    pub max_rw_session_count: c_ulong,
    pub rw_session_count: c_ulong,
    pub max_pin_len: c_ulong,
    pub min_pin_len: c_ulong,
    pub total_public_memory: c_ulong,
    pub free_public_memory: c_ulong,
    pub total_private_memory: c_ulong,
    pub free_private_memory: c_ulong,
    pub hardware_version: Version,
    pub firmware_version: Version,
    pub utc_time: [c_uchar; 16usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SessionInfo {
    pub slot_id: c_ulong,
    pub state: c_ulong,
    pub flags: c_ulong,
    pub device_error: c_ulong,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Attribute {
    pub type_: c_ulong,
    pub value: *mut c_void,
    pub value_len: c_ulong,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Date {
    pub year: [c_uchar; 4usize],
    pub month: [c_uchar; 2usize],
    pub day: [c_uchar; 2usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Mechanism {
    pub mechanism: c_ulong,
    pub parameter: *mut c_void,
    pub parameter_len: c_ulong,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MechanismInfo {
    pub min_key_size: c_ulong,
    pub max_key_size: c_ulong,
    pub flags: c_ulong,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CInitializeArgs {
    pub create_mutex: Option<unsafe extern "C" fn(*mut *mut c_void) -> c_ulong>,
    pub destroy_mutex: Option<unsafe extern "C" fn(*mut c_void) -> c_ulong>,
    pub lock_mutex: Option<unsafe extern "C" fn(*mut c_void) -> c_ulong>,
    pub unlockmutex: Option<unsafe extern "C" fn(*mut c_void) -> c_ulong>,
    pub flags: c_ulong,
    pub reserved: *mut c_void,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RsaPkcsOaepParams {
    pub hash_alg: c_ulong,
    pub mgf: c_ulong,
    pub source: c_ulong,
    pub source_data: *mut c_void,
    pub source_data_len: c_ulong,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RsaPkcsPssParams {
    pub hash_alg: c_ulong,
    pub mgf: c_ulong,
    pub len: c_ulong,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Ecdh1DeriveParams {
    pub kdf: c_ulong,
    pub shared_data_len: c_ulong,
    pub shared_data: *mut c_uchar,
    pub public_data_len: c_ulong,
    pub public_data: *mut c_uchar,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Ecdh2DeriveParams {
    pub kdf: c_ulong,
    pub shared_data_len: c_ulong,
    pub shared_data: *mut c_uchar,
    pub public_data_len: c_ulong,
    pub public_data: *mut c_uchar,
    pub private_data_len: c_ulong,
    pub private_data: c_ulong,
    pub public_data_len2: c_ulong,
    pub public_data2: *mut c_uchar,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct EcmqvDeriveParams {
    pub kdf: c_ulong,
    pub shared_data_len: c_ulong,
    pub shared_data: *mut c_uchar,
    pub public_data_len: c_ulong,
    pub public_data: *mut c_uchar,
    pub private_data_len: c_ulong,
    pub private_data: c_ulong,
    pub public_data_len2: c_ulong,
    pub public_data2: *mut c_uchar,
    pub public_key: c_ulong,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct X942Dh1DeriveParams {
    pub kdf: c_ulong,
    pub other_info_len: c_ulong,
    pub other_info: *mut c_uchar,
    pub public_data_len: c_ulong,
    pub public_data: *mut c_uchar,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct X942Dh2DeriveParams {
    pub kdf: c_ulong,
    pub other_info_len: c_ulong,
    pub other_info: *mut c_uchar,
    pub public_data_len: c_ulong,
    pub public_data: *mut c_uchar,
    pub private_data_len: c_ulong,
    pub private_data: c_ulong,
    pub public_data_len2: c_ulong,
    pub public_data2: *mut c_uchar,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct X942MqvDeriveParams {
    pub kdf: c_ulong,
    pub other_info_len: c_ulong,
    pub other_info: *mut c_uchar,
    pub public_data_len: c_ulong,
    pub public_data: *mut c_uchar,
    pub private_data_len: c_ulong,
    pub private_data: c_ulong,
    pub public_data_len2: c_ulong,
    pub public_data2: *mut c_uchar,
    pub public_key: c_ulong,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct KeaDeriveParams {
    pub is_sender: c_uchar,
    pub random_len: c_ulong,
    pub random_a: *mut c_uchar,
    pub random_b: *mut c_uchar,
    pub public_data_len: c_ulong,
    pub public_data: *mut c_uchar,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Rc2CbcParams {
    pub effective_bits: c_ulong,
    pub iv: [c_uchar; 8usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Rc2MacGeneralParams {
    pub effective_bits: c_ulong,
    pub mac_length: c_ulong,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Rc5Params {
    pub wordsize: c_ulong,
    pub rounds: c_ulong,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Rc5CbcParams {
    pub wordsize: c_ulong,
    pub rounds: c_ulong,
    pub iv: *mut c_uchar,
    pub iv_len: c_ulong,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Rc5MacGeneralParams {
    pub wordsize: c_ulong,
    pub rounds: c_ulong,
    pub mac_length: c_ulong,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DesCbcEncryptDataParams {
    pub iv: [c_uchar; 8usize],
    pub data: *mut c_uchar,
    pub length: c_ulong,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AesCbcEncryptDataParams {
    pub iv: [c_uchar; 16usize],
    pub data: *mut c_uchar,
    pub length: c_ulong,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SkipjackPrivateWrapParams {
    pub password_len: c_ulong,
    pub password: *mut c_uchar,
    pub public_data_len: c_ulong,
    pub public_data: *mut c_uchar,
    pub p_and_g_len: c_ulong,
    pub q_len: c_ulong,
    pub random_len: c_ulong,
    pub random_a: *mut c_uchar,
    pub prime_p: *mut c_uchar,
    pub base_g: *mut c_uchar,
    pub subprime_q: *mut c_uchar,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SkipjackRelayxParams {
    pub old_wrapped_x_len: c_ulong,
    pub old_wrapped_x: *mut c_uchar,
    pub old_password_len: c_ulong,
    pub old_password: *mut c_uchar,
    pub old_public_data_len: c_ulong,
    pub old_public_data: *mut c_uchar,
    pub old_pandom_len: c_ulong,
    pub old_random_a: *mut c_uchar,
    pub new_password_len: c_ulong,
    pub new_password: *mut c_uchar,
    pub new_public_data_len: c_ulong,
    pub new_public_data: *mut c_uchar,
    pub new_random_len: c_ulong,
    pub new_random_a: *mut c_uchar,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PbeParams {
    pub init_vector: *mut c_uchar,
    pub password: *mut c_uchar,
    pub password_len: c_ulong,
    pub salt: *mut c_uchar,
    pub salt_len: c_ulong,
    pub iteration: c_ulong,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct KeyWrapSetOaepParams {
    pub bc: c_uchar,
    pub x: *mut c_uchar,
    pub x_len: c_ulong,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Ssl3RandomData {
    pub client_random: *mut c_uchar,
    pub client_random_len: c_ulong,
    pub server_random: *mut c_uchar,
    pub server_random_len: c_ulong,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Ssl3MasterKeyDeriveParams {
    pub random_info: Ssl3RandomData,
    pub version: *mut Version,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Ssl3KeyMatOut {
    pub client_mac_secret: c_ulong,
    pub server_mac_secret: c_ulong,
    pub client_key: c_ulong,
    pub server_key: c_ulong,
    pub iv_client: *mut c_uchar,
    pub iv_server: *mut c_uchar,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Ssl3KeyMatParams {
    pub mac_size_in_bits: c_ulong,
    pub key_size_in_bits: c_ulong,
    pub iv_size_in_bits: c_ulong,
    pub is_export: c_uchar,
    pub random_info: Ssl3RandomData,
    pub returned_key_material: *mut Ssl3KeyMatOut,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TlsPrfParams {
    pub seed: *mut c_uchar,
    pub seed_len: c_ulong,
    pub label: *mut c_uchar,
    pub label_len: c_ulong,
    pub output: *mut c_uchar,
    pub output_len: *mut c_ulong,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct WtlsRandomData {
    pub client_random: *mut c_uchar,
    pub client_random_len: c_ulong,
    pub server_random: *mut c_uchar,
    pub server_random_len: c_ulong,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct WtlsMasterKeyDeriveParams {
    pub digest_mechanism: c_ulong,
    pub random_info: WtlsRandomData,
    pub version: *mut c_uchar,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct WtlsPrfParams {
    pub digest_mechanism: c_ulong,
    pub seed: *mut c_uchar,
    pub seed_len: c_ulong,
    pub label: *mut c_uchar,
    pub label_len: c_ulong,
    pub output: *mut c_uchar,
    pub output_len: *mut c_ulong,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct WtlsKeyMatOut {
    pub mac_secret: c_ulong,
    pub key: c_ulong,
    pub iv: *mut c_uchar,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct WtlsKeyMatParams {
    pub digest_mechanism: c_ulong,
    pub mac_size_in_bits: c_ulong,
    pub key_size_in_bits: c_ulong,
    pub iv_size_in_bits: c_ulong,
    pub sequence_number: c_ulong,
    pub is_export: c_uchar,
    pub random_info: WtlsRandomData,
    pub returned_key_material: *mut WtlsKeyMatOut,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CmsSigParams {
    pub certificate_handle: c_ulong,
    pub signing_mechanism: *mut Mechanism,
    pub digest_mechanism: *mut Mechanism,
    pub content_type: *mut c_uchar,
    pub requested_attributes: *mut c_uchar,
    pub requested_attributes_len: c_ulong,
    pub required_attributes: *mut c_uchar,
    pub required_attributes_len: c_ulong,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct KeyDerivationStringData {
    pub data: *mut c_uchar,
    pub len: c_ulong,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Pkcs5Pbkd2Params {
    pub salt_source: c_ulong,
    pub salt_source_data: *mut c_void,
    pub salt_source_data_len: c_ulong,
    pub iterations: c_ulong,
    pub prf: c_ulong,
    pub prf_data: *mut c_void,
    pub prf_data_len: c_ulong,
    pub password: *mut c_uchar,
    pub password_len: *mut c_ulong,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Pkcs5Pbkd2Params2 {
    pub salt_source: c_ulong,
    pub salt_source_data: *mut c_void,
    pub salt_source_data_len: c_ulong,
    pub iterations: c_ulong,
    pub prf: c_ulong,
    pub prf_data: *mut c_void,
    pub prf_data_len: c_ulong,
    pub password: *mut c_uchar,
    pub password_len: c_ulong,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OtpParam {
    pub type_: c_ulong,
    pub value: *mut c_void,
    pub value_len: c_ulong,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OtpParams {
    pub params: *mut OtpParam,
    pub count: c_ulong,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OtpSignatureInfo {
    pub params: *mut OtpParam,
    pub count: c_ulong,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct KipParams {
    pub mechanism: *mut Mechanism,
    pub key: c_ulong,
    pub seed: *mut c_uchar,
    pub seed_len: c_ulong,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AesCtrParams {
    pub counter_bits: c_ulong,
    pub cb: [c_uchar; 16usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GcmParams {
    pub iv: *mut c_uchar,
    pub iv_len: c_ulong,
    pub iv_bits: c_ulong,
    pub aad: *mut c_uchar,
    pub aad_len: c_ulong,
    pub tag_bits: c_ulong,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CcmParams {
    pub data_len: c_ulong,
    pub nonce: *mut c_uchar,
    pub nonce_len: c_ulong,
    pub aad: *mut c_uchar,
    pub aad_len: c_ulong,
    pub mac_len: c_ulong,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AesGcmParams {
    pub iv: *mut c_uchar,
    pub iv_len: c_ulong,
    pub iv_bits: c_ulong,
    pub aad: *mut c_uchar,
    pub aad_len: c_ulong,
    pub tag_bits: c_ulong,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AesCcmParams {
    pub data_len: c_ulong,
    pub nonce: *mut c_uchar,
    pub nonce_len: c_ulong,
    pub aad: *mut c_uchar,
    pub aad_len: c_ulong,
    pub mac_len: c_ulong,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CamelliaCtrParams {
    pub counter_bits: c_ulong,
    pub cb: [c_uchar; 16usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CamelliaCbcEncryptDataParams {
    pub iv: [c_uchar; 16usize],
    pub data: *mut c_uchar,
    pub length: c_ulong,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AriaCbcEncryptDataParams {
    pub iv: [c_uchar; 16usize],
    pub data: *mut c_uchar,
    pub length: c_ulong,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DsaParameterGenParam {
    pub hash: c_ulong,
    pub seed: *mut c_uchar,
    pub seed_len: c_ulong,
    pub index: c_ulong,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct EcdhAesKeyWrapParams {
    pub aes_key_bits: c_ulong,
    pub kdf: c_ulong,
    pub shared_data_len: c_ulong,
    pub shared_data: *mut c_uchar,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RsaAesKeyWrapParams {
    pub aes_key_bits: c_ulong,
    pub oaep_params: *mut RsaPkcsOaepParams,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Tls12MasterKeyDeriveParams {
    pub random_info: Ssl3RandomData,
    pub version: *mut Version,
    pub prf_hash_mechanism: c_ulong,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Tls12KeyMatParams {
    pub mac_size_in_bits: c_ulong,
    pub key_size_in_bits: c_ulong,
    pub iv_size_in_bits: c_ulong,
    pub is_export: c_uchar,
    pub random_info: Ssl3RandomData,
    pub returned_key_material: *mut Ssl3KeyMatOut,
    pub hash_mechanism: c_ulong,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TlsKdfParams {
    pub mechanism: c_ulong,
    pub label: *mut c_uchar,
    pub label_length: c_ulong,
    pub random_info: Ssl3RandomData,
    pub context_data: *mut c_uchar,
    pub context_data_length: c_ulong,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TlsMacParams {
    pub hash_mechanism: c_ulong,
    pub mac_length: c_ulong,
    pub server_or_client: c_ulong,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Gostr3410DeriveParams {
    pub kdf: c_ulong,
    pub public_data: *mut c_uchar,
    pub public_data_len: c_ulong,
    pub ukm: *mut c_uchar,
    pub ukm_len: c_ulong,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Gostr3410KeyWrapParams {
    pub wrap_oid: *mut c_uchar,
    pub wrap_oid_len: c_ulong,
    pub ukm: *mut c_uchar,
    pub ukm_len: c_ulong,
    pub key: c_ulong,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SeedCbcEncryptDataParams {
    pub iv: [c_uchar; 16usize],
    pub data: *mut c_uchar,
    pub length: c_ulong,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FunctionList {
    pub version: Version,
    pub initialize: Option<unsafe extern "C" fn(*mut c_void) -> c_ulong>,
    pub finalize: Option<unsafe extern "C" fn(*mut c_void) -> c_ulong>,
    pub get_info: Option<unsafe extern "C" fn(*mut Info) -> c_ulong>,
    pub get_function_list: Option<unsafe extern "C" fn(*mut *mut FunctionList) -> c_ulong>,
    pub get_slot_list: Option<unsafe extern "C" fn(c_uchar, *mut c_ulong, *mut c_ulong) -> c_ulong>,
    pub get_slot_info: Option<unsafe extern "C" fn(c_ulong, *mut SlotInfo) -> c_ulong>,
    pub get_token_info: Option<unsafe extern "C" fn(c_ulong, *mut TokenInfo) -> c_ulong>,
    pub get_mechanism_list:
        Option<unsafe extern "C" fn(c_ulong, *mut c_ulong, *mut c_ulong) -> c_ulong>,
    pub get_mechanism_info:
        Option<unsafe extern "C" fn(c_ulong, c_ulong, *mut MechanismInfo) -> c_ulong>,
    pub init_token:
        Option<unsafe extern "C" fn(c_ulong, *mut c_uchar, c_ulong, *mut c_uchar) -> c_ulong>,
    pub init_pin: Option<unsafe extern "C" fn(c_ulong, *mut c_uchar, c_ulong) -> c_ulong>,
    pub set_pin: Option<
        unsafe extern "C" fn(c_ulong, *mut c_uchar, c_ulong, *mut c_uchar, c_ulong) -> c_ulong,
    >,
    pub open_session: Option<
        unsafe extern "C" fn(
            c_ulong,
            c_ulong,
            *mut c_void,
            Option<unsafe extern "C" fn(c_ulong, c_ulong, *mut c_void) -> c_ulong>,
            *mut c_ulong,
        ) -> c_ulong,
    >,
    pub close_session: Option<unsafe extern "C" fn(c_ulong) -> c_ulong>,
    pub close_all_sessions: Option<unsafe extern "C" fn(c_ulong) -> c_ulong>,
    pub get_session_info: Option<unsafe extern "C" fn(c_ulong, *mut SessionInfo) -> c_ulong>,
    pub get_operation_state:
        Option<unsafe extern "C" fn(c_ulong, *mut c_uchar, *mut c_ulong) -> c_ulong>,
    pub set_operation_state:
        Option<unsafe extern "C" fn(c_ulong, *mut c_uchar, c_ulong, c_ulong, c_ulong) -> c_ulong>,
    pub login: Option<unsafe extern "C" fn(c_ulong, c_ulong, *mut c_uchar, c_ulong) -> c_ulong>,
    pub logout: Option<unsafe extern "C" fn(c_ulong) -> c_ulong>,
    pub create_object:
        Option<unsafe extern "C" fn(c_ulong, *mut Attribute, c_ulong, *mut c_ulong) -> c_ulong>,
    pub copy_object: Option<
        unsafe extern "C" fn(c_ulong, c_ulong, *mut Attribute, c_ulong, *mut c_ulong) -> c_ulong,
    >,
    pub destroy_object: Option<unsafe extern "C" fn(c_ulong, c_ulong) -> c_ulong>,
    pub get_object_size: Option<unsafe extern "C" fn(c_ulong, c_ulong, *mut c_ulong) -> c_ulong>,
    pub get_attribute_value:
        Option<unsafe extern "C" fn(c_ulong, c_ulong, *mut Attribute, c_ulong) -> c_ulong>,
    pub set_attribute_value:
        Option<unsafe extern "C" fn(c_ulong, c_ulong, *mut Attribute, c_ulong) -> c_ulong>,
    pub find_objects_init:
        Option<unsafe extern "C" fn(c_ulong, *mut Attribute, c_ulong) -> c_ulong>,
    pub find_objects:
        Option<unsafe extern "C" fn(c_ulong, *mut c_ulong, c_ulong, *mut c_ulong) -> c_ulong>,
    pub find_objects_final: Option<unsafe extern "C" fn(c_ulong) -> c_ulong>,
    pub encrypt_init: Option<unsafe extern "C" fn(c_ulong, *mut Mechanism, c_ulong) -> c_ulong>,
    pub encrypt: Option<
        unsafe extern "C" fn(c_ulong, *mut c_uchar, c_ulong, *mut c_uchar, *mut c_ulong) -> c_ulong,
    >,
    pub encrypt_update: Option<
        unsafe extern "C" fn(c_ulong, *mut c_uchar, c_ulong, *mut c_uchar, *mut c_ulong) -> c_ulong,
    >,
    pub encrypt_final: Option<unsafe extern "C" fn(c_ulong, *mut c_uchar, *mut c_ulong) -> c_ulong>,
    pub decrypt_init: Option<unsafe extern "C" fn(c_ulong, *mut Mechanism, c_ulong) -> c_ulong>,
    pub decrypt: Option<
        unsafe extern "C" fn(c_ulong, *mut c_uchar, c_ulong, *mut c_uchar, *mut c_ulong) -> c_ulong,
    >,
    pub decrypt_update: Option<
        unsafe extern "C" fn(c_ulong, *mut c_uchar, c_ulong, *mut c_uchar, *mut c_ulong) -> c_ulong,
    >,
    pub decrypt_final: Option<unsafe extern "C" fn(c_ulong, *mut c_uchar, *mut c_ulong) -> c_ulong>,
    pub digest_init: Option<unsafe extern "C" fn(c_ulong, *mut Mechanism) -> c_ulong>,
    pub digest: Option<
        unsafe extern "C" fn(c_ulong, *mut c_uchar, c_ulong, *mut c_uchar, *mut c_ulong) -> c_ulong,
    >,
    pub digest_update: Option<unsafe extern "C" fn(c_ulong, *mut c_uchar, c_ulong) -> c_ulong>,
    pub digest_key: Option<unsafe extern "C" fn(c_ulong, c_ulong) -> c_ulong>,
    pub digest_final: Option<unsafe extern "C" fn(c_ulong, *mut c_uchar, *mut c_ulong) -> c_ulong>,
    pub sign_init: Option<unsafe extern "C" fn(c_ulong, *mut Mechanism, c_ulong) -> c_ulong>,
    pub sign: Option<
        unsafe extern "C" fn(c_ulong, *mut c_uchar, c_ulong, *mut c_uchar, *mut c_ulong) -> c_ulong,
    >,
    pub sign_update: Option<unsafe extern "C" fn(c_ulong, *mut c_uchar, c_ulong) -> c_ulong>,
    pub sign_final: Option<unsafe extern "C" fn(c_ulong, *mut c_uchar, *mut c_ulong) -> c_ulong>,
    pub sign_recover_init:
        Option<unsafe extern "C" fn(c_ulong, *mut Mechanism, c_ulong) -> c_ulong>,
    pub sign_recover: Option<
        unsafe extern "C" fn(c_ulong, *mut c_uchar, c_ulong, *mut c_uchar, *mut c_ulong) -> c_ulong,
    >,
    pub verify_init: Option<unsafe extern "C" fn(c_ulong, *mut Mechanism, c_ulong) -> c_ulong>,
    pub verify: Option<
        unsafe extern "C" fn(c_ulong, *mut c_uchar, c_ulong, *mut c_uchar, c_ulong) -> c_ulong,
    >,
    pub verify_update: Option<unsafe extern "C" fn(c_ulong, *mut c_uchar, c_ulong) -> c_ulong>,
    pub verify_final: Option<unsafe extern "C" fn(c_ulong, *mut c_uchar, c_ulong) -> c_ulong>,
    pub verify_recover_init:
        Option<unsafe extern "C" fn(c_ulong, *mut Mechanism, c_ulong) -> c_ulong>,
    pub verify_recover: Option<
        unsafe extern "C" fn(c_ulong, *mut c_uchar, c_ulong, *mut c_uchar, *mut c_ulong) -> c_ulong,
    >,
    pub digest_encrypt_update: Option<
        unsafe extern "C" fn(c_ulong, *mut c_uchar, c_ulong, *mut c_uchar, *mut c_ulong) -> c_ulong,
    >,
    pub decrypt_digest_update: Option<
        unsafe extern "C" fn(c_ulong, *mut c_uchar, c_ulong, *mut c_uchar, *mut c_ulong) -> c_ulong,
    >,
    pub sign_encrypt_update: Option<
        unsafe extern "C" fn(c_ulong, *mut c_uchar, c_ulong, *mut c_uchar, *mut c_ulong) -> c_ulong,
    >,
    pub decrypt_verify_update: Option<
        unsafe extern "C" fn(c_ulong, *mut c_uchar, c_ulong, *mut c_uchar, *mut c_ulong) -> c_ulong,
    >,
    pub generate_key: Option<
        unsafe extern "C" fn(
            c_ulong,
            *mut Mechanism,
            *mut Attribute,
            c_ulong,
            *mut c_ulong,
        ) -> c_ulong,
    >,
    pub generate_key_pair: Option<
        unsafe extern "C" fn(
            c_ulong,
            *mut Mechanism,
            *mut Attribute,
            c_ulong,
            *mut Attribute,
            c_ulong,
            *mut c_ulong,
            *mut c_ulong,
        ) -> c_ulong,
    >,
    pub wrap_key: Option<
        unsafe extern "C" fn(
            c_ulong,
            *mut Mechanism,
            c_ulong,
            c_ulong,
            *mut c_uchar,
            *mut c_ulong,
        ) -> c_ulong,
    >,
    pub unwrap_key: Option<
        unsafe extern "C" fn(
            c_ulong,
            *mut Mechanism,
            c_ulong,
            *mut c_uchar,
            c_ulong,
            *mut Attribute,
            c_ulong,
            *mut c_ulong,
        ) -> c_ulong,
    >,
    pub derive_key: Option<
        unsafe extern "C" fn(
            c_ulong,
            *mut Mechanism,
            c_ulong,
            *mut Attribute,
            c_ulong,
            *mut c_ulong,
        ) -> c_ulong,
    >,
    pub seed_random: Option<unsafe extern "C" fn(c_ulong, *mut c_uchar, c_ulong) -> c_ulong>,
    pub generate_random: Option<unsafe extern "C" fn(c_ulong, *mut c_uchar, c_ulong) -> c_ulong>,
    pub get_function_status: Option<unsafe extern "C" fn(c_ulong) -> c_ulong>,
    pub cancel_function: Option<unsafe extern "C" fn(c_ulong) -> c_ulong>,
    pub wait_for_slot_event:
        Option<unsafe extern "C" fn(c_ulong, *mut c_ulong, *mut c_void) -> c_ulong>,
}
