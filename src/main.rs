extern crate base64;
extern crate byteorder;
extern crate hex;
extern crate rand;
extern crate serde_json;
extern crate signal_hook;

use std::collections::HashMap;
use std::io::{Read, Write, Result};
use std::os::unix::net::{UnixStream, UnixListener};
use std::sync::{Arc, Mutex};
use std::thread;
use byteorder::{BE, ReadBytesExt, WriteBytesExt};

mod bridge;
mod key;
mod list;
mod sign;
mod startup;
mod store;

// defined in the specification document
const SSH_AGENTC_REQUEST_IDENTITIES: u8 = 11;
const SSH_AGENTC_SIGN_REQUEST: u8 = 13;
const SSH_AGENT_FAILURE: u8 = 5;

type KeyStore = HashMap<Vec<u8>, key::Key>;

fn main() {
    // check if "-h" is passed, print help and exit if so
    let help_requested = startup::print_help();
    if help_requested { return; }

    let foreground_mode = startup::is_foreground();
    let socket = startup::get_socket();

    let pid = if foreground_mode {
        // if we are running in the foreground mode, set up the
        // socket, and use our own PID
        startup::initialize_socket(&socket);
        startup::get_pid()
    } else {
        // if not, spawn a child that runs in foreground mode,
        // and use its PID
        startup::spawn_child(&socket)
            .expect("Could not execute a child process.")
    };

    // print the greetings: the socket and PID information
    println!("{}", startup::create_shell_commands(&socket, pid));
    // child will take over if we are not in foreground mode
    if !foreground_mode { return; }

    // keystore cache that will be used throughout the application
    let store = Arc::new(Mutex::new(KeyStore::new()));

    let listener = UnixListener::bind(socket).expect("Could not create the socket.");

    for stream in listener.incoming() {
        if let Ok(mut stream) = stream {
            let store = Arc::clone(&store);
            thread::spawn(move || {
                loop {
                    if let Err(_) = handle_stream(&store, &mut stream) {
                        break;
                    };
                }
            });
        }
    }
}

fn handle_stream(store: &Arc<Mutex<KeyStore>>, stream: &mut UnixStream) -> Result<()> {
    let length = stream.read_u32::<BE>()?;
    let msg_type = stream.read_u8()?;
    match msg_type {
        SSH_AGENTC_REQUEST_IDENTITIES => {
            let mut store = store.lock().unwrap();

            // grab the keys from termux-api
            let json = bridge::list_keys();

            // parse the JSON to refresh the local cache
            store::load_all(&mut store, json);

            // print the keys from the cache
            list::write_list_response(&store, stream)?;
        },
        SSH_AGENTC_SIGN_REQUEST => {
            let mut store = store.lock().unwrap();

            // create a separate stream for reading
            let mut read = stream.try_clone().unwrap().take((length - 1).into());

            // parse the request received from the client
            let request = match sign::read_request(&store, &mut read)? {
                Some(request) => request,
                _ => { return write_error(stream) },
            };

            // transmit the request to termux-api
            let signature = bridge::sign(
                &request.key().alias, &request.keystore_name(), &request.data()
            );

            if signature.len() == 0 {
                // termux-api returned empty response
                // it is probably locked due to user validity enforcement
                return write_error(stream);
            }

            // print the signature response to the client
            sign::write_response(&request, &signature, stream)?;
            stream.flush()?;
        },
        _ => {
            // unsupported message type
            write_error(stream)?;
        },
    };
    Ok(())
}

fn write_error(stream: &mut Write) -> Result<()> {
    stream.write_u32::<BE>(1)?;
    stream.write_u8(SSH_AGENT_FAILURE)?;
    stream.flush()?;
    Ok(())
}
