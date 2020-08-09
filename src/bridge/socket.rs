//! Provides a struct which can be used to create and use Unix abstract sockets.

use std::error::Error;
use std::io::{self, ErrorKind, Read, Write};

use nix::sys::socket::{self, AddressFamily, SockAddr, SockFlag, SockType, UnixAddr};
use nix::unistd;

use uuid::adapter::Hyphenated;
use uuid::Uuid;

/// Represents an abstract Unix socket.
pub struct Socket {
    address: SockAddr,
    server_socket: i32,
    client_socket: Option<i32>,
}

impl Socket {
    /// Creates a new abstract Unix socket with a randomly generated address,
    /// binds to it and starts listening on this address.
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let mut buf = [0u8; Hyphenated::LENGTH];
        Uuid::new_v4().to_hyphenated().encode_lower(&mut buf);
        let address = UnixAddr::new_abstract(&buf)?;
        let address = SockAddr::Unix(address);
        let server_socket = socket::socket(
            AddressFamily::Unix,
            SockType::Stream,
            SockFlag::SOCK_CLOEXEC,
            None,
        )?;
        socket::bind(server_socket, &address)?;
        socket::listen(server_socket, 1)?;
        Ok(Socket {
            address,
            server_socket,
            client_socket: None,
        })
    }

    /// Accept a connection through this socket. This will block until
    /// the other side of the socket connects to this socket, if not
    /// connected already.
    pub fn accept(&mut self) -> Result<(), Box<dyn Error>> {
        let client_socket = socket::accept(self.server_socket)?;
        self.client_socket = Some(client_socket);
        Ok(())
    }

    /// Closes this socket. It will be possible to accept another
    /// connection after this.
    pub fn close(&mut self) -> Result<(), Box<dyn Error>> {
        unistd::close(self.client_socket()?)?;
        self.client_socket = None;
        Ok(())
    }

    /// Returns the address of this socket. It will not have a leading
    /// null byte nor an "@" symbol.
    pub fn address(&self) -> String {
        let mut address = self.address.to_str();
        address.remove(0);
        address
    }

    /// Returns the client socket associated with this socket.
    /// Returns `None` if the socket has not accepted a connection yet.
    fn client_socket(&self) -> io::Result<i32> {
        Ok(self.client_socket.ok_or(io::Error::new(
            ErrorKind::NotConnected,
            self.address.to_str(),
        ))?)
    }
}

impl Read for Socket {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        unistd::read(self.client_socket()?, buf).map_err(to_io_err)
    }
}

impl Write for Socket {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        unistd::write(self.client_socket()?, buf).map_err(to_io_err)
    }

    fn flush(&mut self) -> io::Result<()> {
        // It is not possible to implement this with Unix abstract sockets.
        Ok(())
    }
}

/// Converts a [`nix::Error`] to [`io::Error`].
///
/// [`nix::Error`]: https://docs.rs/nix/0.18.0/nix/enum.Error.html
/// [`io::Error`]: https://doc.rust-lang.org/std/io/struct.Error.html
fn to_io_err(err: nix::Error) -> io::Error {
    io::Error::new(ErrorKind::Other, err)
}
