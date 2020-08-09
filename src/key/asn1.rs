//! Implementations for parsing and creating ASN.1 encoded documents.
//! All ASN.1 related functions must reside in this module.

use std::iter;

use simple_asn1::*;

use super::{EcCurve, EcKey};

/// For elliptic curves, 2 and 3 means compressed and 4 means uncompressed.
/// We only support the uncompressed point representation.
const EC_UNCOMPRESSED_POINT: u8 = 4;

impl EcCurve {
    /// Returns the required parameter length for this curve. All values
    /// (x, y, r and s) must be prepended with zeroes so that they have this length.
    fn param_length(&self) -> usize {
        match &self {
            EcCurve::P256 => 32,
            EcCurve::P384 => 48,
            EcCurve::P521 => 66,
        }
    }
}

impl EcKey {
    /// Returns the point values (x and y), which is the public key of this EC key.
    /// The output is formatted in ASN.1.
    pub fn point_as_asn1(&self) -> Result<Vec<u8>, ASN1EncodeErr> {
        let x_prepend = &self.curve.param_length() - self.x.len();
        let y_prepend = &self.curve.param_length() - self.y.len();

        let point = iter::once(EC_UNCOMPRESSED_POINT)
            .chain(iter::repeat(0).take(x_prepend))
            .chain(self.x.iter().copied())
            .chain(iter::repeat(0).take(y_prepend))
            .chain(self.y.iter().copied())
            .collect();

        let asn = ASN1Block::OctetString(0, point);
        to_der(&asn)
    }

    /// Returns the params of this EC key, which represents the curve used in this key.
    /// The output is formatted in ASN.1.
    pub fn params_as_asn1(&self) -> Result<Vec<u8>, ASN1EncodeErr> {
        let oid = match self.curve {
            EcCurve::P256 => oid!(1, 2, 840, 10045, 3, 1, 7),
            EcCurve::P384 => oid!(1, 3, 132, 0, 34),
            EcCurve::P521 => oid!(1, 3, 132, 0, 35),
        };
        let asn = ASN1Block::ObjectIdentifier(0, oid);
        to_der(&asn)
    }

    /// Parses a signature value formatted in ASN.1, and returns a vector that contains
    /// the signature values (r and s), with proper padding so that it has the expected
    /// total length (2*`param_length()`).
    pub fn signature_from_asn1(&self, signature: &[u8]) -> Option<Vec<u8>> {
        let asn = from_der(signature).ok()?;
        // First item in the signature will contain two values.
        let first = asn.into_iter().next()?;
        let block = match first {
            ASN1Block::Sequence(_, block) => block,
            _ => {
                return None;
            }
        };
        // First one is the r value.
        let r = match &block[0] {
            ASN1Block::Integer(_, one) => one,
            _ => {
                return None;
            }
        };
        // Second one is the s value.
        let s = match &block[1] {
            ASN1Block::Integer(_, two) => two,
            _ => {
                return None;
            }
        };
        let r = r.to_bytes_be().1;
        let s = s.to_bytes_be().1;
        // Prepend to the required length.
        let r_prepend = &self.curve.param_length() - r.len();
        let s_prepend = &self.curve.param_length() - s.len();
        Some(
            iter::repeat(0)
                .take(r_prepend)
                .chain(r)
                .chain(iter::repeat(0).take(s_prepend))
                .chain(s)
                .collect(),
        )
    }
}
