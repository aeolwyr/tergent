//! Provides the bridge to the termux-api. All communication with the
//! termux-api must go through this module.

mod socket;

use std::error::Error;
use std::io::{Read, Write};
use std::process::{Command, Stdio};

use base64;

/// Send a request to `termux-api` to list all the keys.
/// Returns a string that contains a JSON object.
pub fn list_keys() -> Result<String, Box<dyn Error>> {
    Ok(communicate(&["list", "--ez", "detailed", "true"], &[0; 0])?)
}

/// Send some data to `termux-api` to be signed.
/// Algorithm parameter must be in the format that keystore expects
/// (e.g. "SHA512withRSA"). See the full list at the
/// Java documentation for [Signature algorithms].
/// Returns the signature of the data provided.
///
/// [Signature algorithms]:
/// https://docs.oracle.com/javase/8/docs/technotes/guides/security/StandardNames.html#Signature
pub fn sign(alias: &str, algorithm: &str, data: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
    let args = ["sign", "-e", "alias", alias, "-e", "algorithm", algorithm];
    let output = communicate(&args, data)?;
    return Ok(base64::decode(output)?);
}

/// Performs a generic call to `termux-api`, providing `args` to its receiver.
/// Sets up proper sockets so that the `input` is provided to `termux-api` and
/// its output is returned from this function.
fn communicate(args: &[&str], input: &[u8]) -> Result<String, Box<dyn Error>> {
    let mut input_socket = socket::Socket::new()?;
    let mut output_socket = socket::Socket::new()?;

    // This executable does not use stdin/stdout itself.
    let mut command = Command::new("/data/data/com.termux/files/usr/bin/am")
        .arg("broadcast")
        .args(&["--user", "0"])
        .args(&["-n", "com.termux.api/.TermuxApiReceiver"])
        .args(&["--es", "socket_input", &output_socket.address()])
        .args(&["--es", "socket_output", &input_socket.address()])
        .args(&["--es", "api_method", "Keystore", "-e", "command"])
        .args(args)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?;

    input_socket.accept()?;

    // Do not accept on the output socket if there is nothing to send.
    // This is important as it will hang if termux-api is not expecting to receive any input.
    if input.len() > 0 {
        output_socket.accept()?;
        output_socket.write(input)?;
        output_socket.close()?;
    }

    let mut output = String::new();
    input_socket.read_to_string(&mut output)?;
    input_socket.close()?;

    // We need to reap our children otherwise they will stay as zombies.
    command.wait()?;
    Ok(output)
}
