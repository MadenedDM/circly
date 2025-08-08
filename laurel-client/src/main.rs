#![allow(clippy::multiple_crate_versions)]
use std::{
    error::Error,
    io::{self, ErrorKind, Write},
    net::{Ipv4Addr, Shutdown, SocketAddr, SocketAddrV4, TcpStream},
};

const PORT: u16 = 9878;

pub struct Client {
    pub addr: SocketAddr,
    pub stream: TcpStream,
}

impl Client {
    /// Constructs a new client or dies trying
    ///
    /// # Panics
    /// Whenever `try_new` would error
    #[must_use]
    pub fn new(addr: SocketAddr) -> Self {
        match Self::try_new(addr) {
            Ok(dat) => dat,
            Err(e) => match e.kind() {
                ErrorKind::ConnectionRefused => {
                    panic!("Connection Refused, Is the Server up?")
                }
                _ => {
                    panic!("Connection Failed: {e:?}")
                }
            },
        }
    }

    /// Tries to construct a new client
    ///
    /// # Errors
    /// If the server cannot be connected to.
    pub fn try_new(addr: SocketAddr) -> Result<Self, io::Error> {
        let stream = match TcpStream::connect(addr) {
            Ok(dat) => dat,
            Err(e) => {
                return Err(e);
            }
        };

        Ok(Self { addr, stream })
    }

    /// # Errors
    /// Sometimes
    pub fn run(&mut self) -> Result<(), io::Error> {
        let a = self.stream.write_all(&[0; 17]);

        let _ = self.stream.shutdown(Shutdown::Both);
        a
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let address: SocketAddr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), PORT));

    let mut client: Client = match Client::try_new(address) {
        Ok(c) => c,
        Err(e) => match e.kind() {
            ErrorKind::ConnectionRefused => {
                println!("Connection Refused, Is the Server up?");
                return Ok(());
            }
            _ => {
                panic!("Connection Failed: {e:?}")
            }
        },
    };

    client.run()?;

    Ok(())
}
