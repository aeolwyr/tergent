//! Functions related to parsing JSON output, which is vended
//! by termux keystore. JSON related code must be contained in this module.

use hex;
use serde_json;

use super::{EcCurve, EcKey, Key, RsaKey};

/// Convents a JSON string to key vector. Returns `None` if the overall
/// structure is invalid. Skips over the keys that are unparseable.
pub fn to_list(json: String) -> Option<Vec<Key>> {
    let keys = serde_json::from_str::<serde_json::Value>(&json).ok()?;
    let keys = keys.as_array()?;

    Some(keys.iter().filter_map(parse_key).collect())
}

/// Parse a single JSON object containing information about a key.
fn parse_key(object: &serde_json::Value) -> Option<Key> {
    let alias = object.get("alias")?.as_str()?;
    let algorithm = object.get("algorithm")?.as_str()?;
    let size = object.get("size")?.as_u64()?;

    match algorithm {
        "RSA" => {
            let modulus = object.get("modulus")?.as_hex()?;
            let exponent = object.get("exponent")?.as_hex()?;
            let key = RsaKey {
                label: String::from(alias),
                modulus,
                exponent,
            };
            Some(Key::Rsa(key))
        }
        "EC" => {
            let x = object.get("x")?.as_hex()?;
            let y = object.get("y")?.as_hex()?;
            let curve = match size {
                256 => EcCurve::P256,
                384 => EcCurve::P384,
                521 => EcCurve::P521,
                _ => {
                    return None;
                }
            };
            let key = EcKey {
                label: String::from(alias),
                curve,
                x,
                y,
            };
            Some(Key::Ec(key))
        }
        _ => None,
    }
}

trait HexValue {
    fn as_hex(&self) -> Option<Vec<u8>>;
}

/// Extend the JSON value struct to support parsing base16 integers.
impl HexValue for serde_json::Value {
    fn as_hex(&self) -> Option<Vec<u8>> {
        let value = self.as_str()?;

        if value.len() % 2 == 1 {
            // hex module will fail if the number of letters is odd
            // prepend a zero to get the result without failing
            hex::decode(format!("0{}", value)).ok()
        } else {
            hex::decode(value).ok()
        }
    }
}
