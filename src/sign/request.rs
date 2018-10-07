use ::key::{Key, Algorithm, Curve};

// defined in the specification document
const SSH_AGENT_RSA_SHA2_256: u32 = 2;
const SSH_AGENT_RSA_SHA2_512: u32 = 4;

/// Represents a request for signing send by a client.
pub struct SignRequest<'a> {
    key: &'a Key,
    data: Vec<u8>,
    flags: u32,
}

impl<'a> SignRequest<'a> {
    pub fn new(key: &Key, data: Vec<u8>, flags: u32) -> SignRequest {
        SignRequest { key, data, flags }
    }

    pub fn key(&self) -> &Key {
        &self.key
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    /// Returns the tuple of keystore name and ssh name for this key.
    fn name(&self) -> (&'static str, &'static str) {
        match self.key.algorithm {
            Algorithm::Rsa => {
                // other implementations also have this order
                if self.flags & SSH_AGENT_RSA_SHA2_256 != 0 {
                    ("SHA256withRSA", "rsa-sha2-256")
                } else if self.flags & SSH_AGENT_RSA_SHA2_512 != 0 {
                    ("SHA512withRSA", "rsa-sha2-512")
                } else {
                    ("SHA1withRSA", "ssh-rsa")
                }
            },
            Algorithm::Ec(ref curve) => {
                match curve {
                    Curve::P256 => ("SHA256withECDSA", "ecdsa-sha2-nistp256"),
                    Curve::P384 => ("SHA386withECDSA", "ecdsa-sha2-nistp384"),
                    Curve::P521 => ("SHA512withECDSA", "ecdsa-sha2-nistp521"),
                }
            },
        }
    }

    /// Returns the name to be used when communicating with the
    /// Android keystore.
    pub fn keystore_name(&self) -> &'static str {
        self.name().0
    }

    /// Returns the name to be used when communicating with an
    /// ssh-agent client.
    pub fn ssh_name(&self) -> &'static str {
        self.name().1
    }
}
