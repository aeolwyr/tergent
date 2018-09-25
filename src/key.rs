/// A key instance, representing a key stored inside the keystore.
pub struct Key {
    pub algorithm: Algorithm,
    pub alias: String,
}

pub enum Algorithm {
    Rsa, Ec(Curve),
}

/// Available elliptic curves, each representing a NIST P curve.
pub enum Curve {
    P256, P384, P521,
}

impl Algorithm {
    /// Tries to find the corrensponding algorithm given the fields
    /// acquired from `termux-api`. Returns an empty response if
    /// either the algorithm is unknown or the key size is incompatible
    /// with the algorithm.
    pub fn parse(algorithm: &str, size: u64) -> Option<Algorithm> {
        match algorithm {
            // In SSH, RSA only has one key type that represents all sizes.
            "RSA" => Some(Algorithm::Rsa),
            "EC" => {
                let curve = match size {
                    256 => Curve::P256,
                    384 => Curve::P384,
                    521 => Curve::P521,
                    _ => return None, // unknown curve
                };
                Some(Algorithm::Ec(curve))
            },
            _ => None, // unknown algorithm
        }
    }
}

impl Curve {
    /// Returns the full curve name, e.g. "nistp256".
    pub fn name(&self) -> &'static str {
        match self {
            Curve::P256 => "nistp256",
            Curve::P384 => "nistp384",
            Curve::P521 => "nistp521",
        }
    }
}
