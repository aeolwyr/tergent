mod request;
mod response;

use std::io::{Read, Write, Result};
use byteorder::{BE, ReadBytesExt, WriteBytesExt};

use super::KeyStore;
use super::key::Algorithm;
use self::request::SignRequest;

/// Parses a request to sign some data, and returns a struct
/// that represents this request. Returns None if the
/// corresponding key was not found in the keystore.
pub fn read_request<'a>(store: &'a KeyStore, stream: &mut Read) -> Result<Option<SignRequest<'a>>> {
    // total length and message type is already read by the main function
    // next up, blob length
    let blob_length = stream.read_u32::<BE>()?;
    let mut blob: Vec<u8> = Vec::new();
    blob.write_u32::<BE>(blob_length)?;
    stream.take(blob_length.into()).read_to_end(&mut blob)?;

    // data to be signed
    let data_length = stream.read_u32::<BE>()?;
    let mut data: Vec<u8> = Vec::new();
    stream.take(data_length.into()).read_to_end(&mut data)?;

    // finally, flags
    let flags = stream.read_u32::<BE>()?;

    // while SSH agent protocol relies on blobs heavily
    // it is easier to work with key structs in our context
    if let Some(key) = store.get(blob.as_slice()) {
        Ok(Some(SignRequest::new(key, data, flags)))
    } else {
        // key not found
        Ok(None)
    }
}

/// Outputs a sign response given a key and a signature received from `termux-api`.
pub fn write_response(request: &SignRequest, signature: &[u8], stream: &mut Write) -> Result<()> {
    let name = request.ssh_name(); // name to use in the header
    match &request.key().algorithm {
        Algorithm::Rsa => {
            response::write_rsa_response(name, signature, stream)
        },
        Algorithm::Ec(_) => {
            // curve is already in name
            response::write_ec_response(name, signature, stream)
        },
    }
}
