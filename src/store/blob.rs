use std::io::{Write, Result};
use byteorder::{BE, WriteBytesExt};

use ::key::Curve;

const EC_UNCOMPRESSED_POINT: u8 = 4;
// compressed points are not supported

/// Create a RSA key blob with the given modulus.
/// Note that the exponent is hardcoded as 65537 for now.
pub fn from_rsa(modulus: Vec<u8>) -> Result<Vec<u8>> {
    let length = modulus.len() as u32;

    let mut blob = Vec::new();
    // blob total size
    blob.write_u32::<BE>(length + 23)?;
    // header
    blob.write_u32::<BE>(7)?;
    blob.write_all("ssh-rsa".as_bytes())?;
    // exponent
    blob.write_u32::<BE>(3)?;
    blob.write_all(&[1, 0, 1])?;
    // modulus
    blob.write_u32::<BE>(length + 1)?;
    blob.write_u8(0)?; // modulus is positive (see https://crypto.stackexchange.com/q/30608)
    blob.write_all(&modulus)?;
    Ok(blob)
}

/// Create an EC key blob with the given parameters.
pub fn from_ec(curve: &Curve, x: Vec<u8>, y: Vec<u8>) -> Result<Vec<u8>> {
    // both parameters must have these lengths
    let xy_length = match curve {
        Curve::P256 => 32,
        Curve::P384 => 48,
        Curve::P521 => 66,
    };
    // will need to prepend with zeroes to make sure
    // the parameters have these sizes
    let x_padding = xy_length as usize - x.len();
    let y_padding = xy_length as usize - y.len();

    // curve name, e.g. "nistp256"
    let curve = curve.name().as_bytes();

    let mut blob = Vec::new();
    // blob total size
    blob.write_u32::<BE>(xy_length * 2 + 40)?;
    // header, algorithm name
    blob.write_u32::<BE>(19)?;
    blob.write_all("ecdsa-sha2-".as_bytes())?;
    blob.write_all(curve)?;
    // curve name
    blob.write_u32::<BE>(8)?;
    blob.write_all(curve)?;
    // parameters
    blob.write_u32::<BE>(xy_length * 2 + 1)?;
    blob.write_u8(EC_UNCOMPRESSED_POINT)?;
    for _ in 0..x_padding { blob.write_u8(0)?; };
    blob.write_all(&x)?;
    for _ in 0..y_padding { blob.write_u8(0)?; };
    blob.write_all(&y)?;
    Ok(blob)
}
