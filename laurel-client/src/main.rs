#![allow(clippy::multiple_crate_versions)]
use std::{
    error::Error,
    io::ErrorKind,
    net::{Ipv4Addr, SocketAddr, SocketAddrV4, TcpStream},
};

const PORT: u16 = 9878;

pub struct Client {
    pub addr: SocketAddr,
    pub stream: TcpStream,
}

impl Client {
    /// Constructs a new client
    ///
    /// # Panics
    /// If the connection is unable to be made, for example if the server is not up.
    #[must_use]
    pub fn new(addr: SocketAddr) -> Self {
        let stream = match TcpStream::connect(addr) {
            Ok(dat) => dat,
            Err(e) => match e.kind() {
                ErrorKind::ConnectionRefused => {
                    panic!("Connection Refused, Is the Server up?")
                }
                _ => {
                    panic!("Connection Failed: {e:?}")
                }
            },
        };

        Self { addr, stream }
    }

    /// # Errors
    /// Sometimes
    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let address: SocketAddr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), PORT));
    let mut client: Client = Client::new(address);

    client.run()?;

    Ok(())
}
