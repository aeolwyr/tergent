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

    // TODO: these two functions below are too similiar...

    /// Returns the name to be used when communicating with the
    /// Android keystore.
    pub fn keystore_name(&self) -> &'static str {
        // TODO: handle invalid flags gracefully
        match self.key.algorithm {
            Algorithm::Rsa => {
                match self.flags {
                    0 => "SHA1withRSA",
                    SSH_AGENT_RSA_SHA2_256 => "SHA256withRSA",
                    SSH_AGENT_RSA_SHA2_512 => "SHA512withRSA",
                    f => panic!("Unknown flag {}", f),
                }
            },
            Algorithm::Ec(ref curve) => {
                match curve {
                    Curve::P256 => "SHA256withECDSA",
                    Curve::P384 => "SHA386withECDSA",
                    Curve::P521 => "SHA512withECDSA",
                }
            },
        }
    }

    /// Returns the name to be used when communicating with an
    /// ssh-agent client.
    pub fn ssh_name(&self) -> &'static str {
        // TODO: handle invalid flags gracefully
        match self.key.algorithm {
            Algorithm::Rsa => {
                match self.flags {
                    0 => "ssh-rsa",
                    SSH_AGENT_RSA_SHA2_256 => "rsa-sha2-256",
                    SSH_AGENT_RSA_SHA2_512 => "rsa-sha2-512",
                    f => panic!("Unknown flag {}", f),
                }
            },
            Algorithm::Ec(ref curve) => {
                match curve {
                    Curve::P256 => "ecdsa-sha2-nistp256",
                    Curve::P384 => "ecdsa-sha2-nistp384",
                    Curve::P521 => "ecdsa-sha2-nistp521",
                }
            },
        }
    }
}
