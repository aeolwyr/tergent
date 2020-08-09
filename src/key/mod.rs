//! Module to represent a single public key. Also provides functionality
//! build the key objects from JSON and to sign using these keys.

mod asn1;
mod json;

pub use json::to_list as json_to_list;

/// A public key instance.
pub enum Key {
    Rsa(RsaKey),
    Ec(EcKey),
}

/// A public RSA key instance.
pub struct RsaKey {
    label: String,
    modulus: Vec<u8>,
    exponent: Vec<u8>,
}

/// A public EC key instance.
pub struct EcKey {
    label: String,
    curve: EcCurve,
    x: Vec<u8>,
    y: Vec<u8>,
}

/// An EC key must use one of these curves.
pub enum EcCurve {
    P256,
    P384,
    P521,
}

impl Key {
    /// Returns the human-readable label of this key.
    pub fn label(&self) -> &str {
        match self {
            Key::Rsa(key) => &key.label,
            Key::Ec(key) => &key.label,
        }
    }

    /// Signs data using this key.
    pub fn sign(&self, data: &[u8]) -> Option<Vec<u8>> {
        match self {
            Key::Rsa(key) => key.sign(data),
            Key::Ec(key) => key.sign(data),
        }
    }
}

impl RsaKey {
    /// Returns the modulus of this key. Its size will be equal to the key length.
    pub fn modulus(&self) -> &[u8] {
        &self.modulus
    }

    /// Returns the exponent of this key, usually 65537.
    pub fn exponent(&self) -> &[u8] {
        &self.exponent
    }

    pub fn sign(&self, data: &[u8]) -> Option<Vec<u8>> {
        super::bridge::sign(&self.label, "NONEwithRSA", data).ok()
    }
}

impl EcKey {
    pub fn sign(&self, data: &[u8]) -> Option<Vec<u8>> {
        let sign = super::bridge::sign(&self.label, "NONEwithECDSA", data).ok()?;
        self.signature_from_asn1(&sign)
    }
}
