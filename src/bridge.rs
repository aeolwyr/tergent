use base64;

use std::io::Write;
use std::process::{Command, Stdio};

/// Location of the `termux-api` application.
const PROGRAM: &str = "/data/data/com.termux/files/usr/libexec/termux-api";

/// Send a request to `termux-api` to list all the keys.
pub fn list_keys() -> String {
    let output = Command::new(PROGRAM)
        .args(&["Keystore", "-e", "command", "list"])
        .args(&["--ez", "detailed", "true"])
        .output()
        .expect("Could not execute the termux-api program.");

    String::from_utf8(output.stdout)
        .expect("Malformed response received from termux-api.")
}

/// Send some data to `termux-api` to be signed.
/// Algorithm parameter must be in the format that keystore expects
/// (e.g. "SHA512withRSA").
pub fn sign(alias: &str, algorithm: &str, data: &[u8]) -> Vec<u8> {
    let mut child = Command::new(PROGRAM)
        .args(&["Keystore", "-e", "command", "sign"])
        .args(&["-e", "alias", alias, "-e", "algorithm", algorithm])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Could not execute the termux-api program.");

    child.stdin.as_mut().and_then(|i| i.write_all(data).ok())
        .expect("Could not send the data to termux-api.");

    child.wait_with_output().ok().and_then(|o| base64::decode(&o.stdout).ok())
        .expect("Could not read the signature response from termux-api.")
}
