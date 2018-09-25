use std::io::{Write, Result};
use byteorder::{BE, WriteBytesExt};

use super::KeyStore;

// defined in the specification document
const SSH_AGENT_IDENTITIES_ANSWER: u8 = 12;

/// Prints the information about all the keys stored in the keystore
/// to the given stream, in the format expected by a ssh-agent client.
pub fn write_list_response(store: &KeyStore, stream: &mut Write) -> Result<()> {
    // total length of the keys
    let length: u32 = store.iter().map(|(blob, key)| {
        // blob already includes its header
        // need 4 for alias header
        (blob.len() as u32) + 4 + (key.alias.len() as u32)
    }).sum();

    // total packet size: +1 for message type and +4 for number of keys
    stream.write_u32::<BE>(length + 5)?;
    // message type
    stream.write_u8(SSH_AGENT_IDENTITIES_ANSWER)?;
    // number of keys
    stream.write_u32::<BE>(store.len() as u32)?;

    for (blob, key) in store {
        // blob (includes size inside)
        stream.write_all(blob)?;
        // alias
        stream.write_u32::<BE>(key.alias.len() as u32)?;
        stream.write_all(key.alias.as_bytes())?;
    }

    Ok(())
}
