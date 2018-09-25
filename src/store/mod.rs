mod blob;

use hex;
use serde_json;
use serde_json::Value;

use super::KeyStore;
use super::key;
use super::key::{Key, Algorithm};

/// Removes all the keys from the store, and fills it back
/// using the data contained in the JSON string.
/// 
/// The input should ideally come from `termux-api`.
pub fn load_all(store: &mut KeyStore, json: String) {
    // expecting a well-formed JSON with an array at the root
    let keys = serde_json::from_str::<Value>(&json).ok()
        .and_then(|v| if let Value::Array(vec) = v { Some(vec) } else { None })
        .expect("Cannot read the JSON input.");

    // panic if any of the objects inside the array is malformed
    let keys = keys.iter()
        .map(|k| parse_key(k).expect("Cannot read one of the keys."));

    store.clear();
    store.extend(keys);
}

/// Parse a single JSON object containing information about a key.
fn parse_key(object: &Value) -> Option<(Vec<u8>, key::Key)> {
    let alias = object.get("alias")?.as_str()?;
    let algorithm = object.get("algorithm")?.as_str()?;
    let size = object.get("size")?.as_u64()?;

    let key = Key {
        algorithm: Algorithm::parse(algorithm, size)?,
        alias: String::from(alias),
    };

    let blob = match key.algorithm {
        Algorithm::Rsa => {
            let modulus = object.get("modulus")?.as_hex()?;
            blob::from_rsa(modulus)
        },
        Algorithm::Ec(ref curve) => {
            let x = object.get("x")?.as_hex()?;
            let y = object.get("y")?.as_hex()?;
            blob::from_ec(curve, x, y)
        },
    }.ok()?;

    Some((blob, key))
}

trait HexValue {
    fn as_hex(&self) -> Option<Vec<u8>>;
}

/// Extend the JSON value struct to support parsing base16 integers.
impl HexValue for Value {
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
