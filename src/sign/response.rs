use std::io::{Write, Result};
use byteorder::{BE, WriteBytesExt};

// defined in the specification document
const SSH_AGENT_SIGN_RESPONSE: u8 = 14;

/// Write a RSA sign response to a given stream.
/// Name should contain the hash algorithm used to sign the data
/// (e.g. "rsa-sha2-512").
pub fn write_rsa_response(name: &str, signature: &[u8], stream: &mut Write) -> Result<()> {
    // the RSA signature received from Android keystore only
    // contains the signature, no parsing is necessary

    let length = signature.len() as u32;

    // total length
    stream.write_u32::<BE>(length + 25)?;
    // message type
    stream.write_u8(SSH_AGENT_SIGN_RESPONSE)?;
    // size for the rest of the packet
    stream.write_u32::<BE>(length + 20)?;
    // signature type
    stream.write_u32::<BE>(12)?;
    stream.write_all(name.as_bytes())?;
    // signature
    stream.write_u32::<BE>(length)?;
    stream.write_all(&signature)?;
    Ok(())
}

/// Write an EC sign response to a given stream.
/// Name should contain the hash algorithm associate with
/// the key size (e.g. "ecdsa-sha2-nistp256")
pub fn write_ec_response(name: &str, signature: &[u8], stream: &mut Write) -> Result<()> {
    // the EC signature received from Android keystore is formatted
    // in ASN.1 DER format.
    // TODO: replace these crude calculations with a proper DER parser
    // first byte is always 0x30
    assert_eq!(0x30, signature[0]);
    // if the second byte is 0x81, we have a longer response (for P-521)
    let r_start = if signature[1] == 0x81 { 3 } else { 2 };
    // 0x02 means integer
    assert_eq!(0x02, signature[r_start]);
    // the next byte is the length of the integer
    let r_length = signature[r_start+1] as usize;
    // repeat for the other value
    let s_start = r_start+2+r_length;
    assert_eq!(0x02, signature[s_start]);
    let s_length = signature[s_start+1] as usize;

    // acquire the values
    let r = &signature[r_start+2..s_start];
    let s = &signature[s_start+2..s_start+2+s_length];

    let rs_length = (r_length + s_length) as u32;

    // total packet size
    stream.write_u32::<BE>(rs_length + 40)?;
    // message type
    stream.write_u8(SSH_AGENT_SIGN_RESPONSE)?;
    // size for the rest of the packet
    stream.write_u32::<BE>(rs_length + 35)?;
    // signature type
    stream.write_u32::<BE>(19)?;
    stream.write_all(name.as_bytes())?;
    // total size of values
    stream.write_u32::<BE>(rs_length + 8)?;
    // values
    stream.write_u32::<BE>(r_length as u32)?;
    stream.write_all(r)?;
    stream.write_u32::<BE>(s_length as u32)?;
    stream.write_all(s)?;
    Ok(())
}
