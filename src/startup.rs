use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::process;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use signal_hook;

/// Root folder for the created socket.
/// A subfolder will be created inside this folder.
const SOCKET_FOLDER: &str = "/data/data/com.termux/files/usr/tmp/";

/// Returns true if the application was started in
/// the foreground mode.
pub fn is_foreground() -> bool {
    env::args().any(|a| a == "-D")
}

/// Get the socket path to use.
/// If available, the path given with the "-a" parameter is used.
/// Otherwise a new path is generated.
pub fn get_socket() -> PathBuf {
    // check if a path is given in the command line
    let mut args = env::args();
    while let Some(arg) = args.next() {
        if arg == "-a" {
            let path = args.next().expect("-a argument requires a path");
            return PathBuf::from(path);
        }
    };

    // generate a new path
    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(12)
        .collect();

    let mut path = PathBuf::from(SOCKET_FOLDER);
    path.push(format!("ssh-{}", rand_string));
    path.push(format!("agent.{}", process::id()));
    path
}

/// Initializes the socket by creating the parent folder,
/// and setting up triggers to cleanup the socket and the folder
/// when the application exists.
pub fn initialize_socket(socket: &Path) {
    // TODO: this function will fail with user-provided socket paths

    socket.parent().and_then(|f| fs::create_dir(f).ok())
        .expect("Could not create the parent folder for the socket");

    register_cleanup(signal_hook::SIGINT, socket);
    register_cleanup(signal_hook::SIGTERM, socket);
}

/// Register a signal hook to remove the folder and the socket.
fn register_cleanup(signal: i32, socket: &Path) {
    let socket = socket.to_owned();
    let folder = socket.parent().unwrap().to_owned();
    
    unsafe {
        signal_hook::register(signal, move || {
            fs::remove_file(&socket).unwrap();
            fs::remove_dir(&folder).unwrap();
            process::exit(0);
        }).unwrap();
    }
}

/// Returns the PID of this process.
pub fn get_pid() -> u32 { process::id() }

/// Executes a child process, with the appropriate parameters
/// passed so that the process reuses the same socket path,
/// and runs in foreground mode. Returns None in case of an error.
pub fn spawn_child(socket: &Path) -> Option<u32> {
    let socket = socket.to_str()?;
    let command = env::args().next()?;
    let child = Command::new(command)
        .args(&["-a", socket, "-D"])
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .spawn().ok()?;
    Some(child.id())
}

/// Builds commands that should be executed in the user's shell
/// so that the socket path and agent PID is easily accessible.
/// If the "-c" is passed, c-shell style commands are generated,
/// if not, bash commands are generated.
pub fn create_shell_commands(socket: &Path, pid: u32) -> String {
    let socket = socket.to_str().expect("Socket path is invalid.");
    let c_style = env::args().any(|a| a == "-c");
    if c_style {
        format!("setenv SSH_AUTH_SOCK {};
setenv SSH_AGENT_PID {};
echo Agent pid {};", socket, pid, pid)
    } else {
        format!("SSH_AUTH_SOCK={}; export SSH_AUTH_SOCK;
SSH_AGENT_PID={}; export SSH_AGENT_PID;
echo Agent pid {};", socket, pid, pid)
    }
}

/// Prints the short help documentation to the console if one of the
/// arguments is "-h". Returns true if this was the case and the help
/// was printed, false otherwise.
pub fn print_help() -> bool {
    if env::args().any(|a| a == "-h") {
        println!("usage: tergent [-c] [-D] [-a bind_address]
Options:
   -c      Print C-shell style commands.
   -D      Foreground mode, do not fork to background.
   -a      Use the given socket address instead of a
           randomly generated one.");
        true
    } else {
        false
    }
}
