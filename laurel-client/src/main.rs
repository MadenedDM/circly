#![allow(clippy::multiple_crate_versions)]
use std::{
    error::Error,
    io::{self, ErrorKind, Read, Write},
    net::{Ipv4Addr, Shutdown, SocketAddr, SocketAddrV4, TcpStream},
};

use laurel_common::lore::{ShakeBuf, gen_client, is_valid_server, new_shake_buf};
use log::{LevelFilter, error, info, warn};
use simplelog::{ColorChoice, CombinedLogger, Config, TermLogger, TerminalMode};

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

    /// Runs the client
    ///
    /// # Errors
    /// If something goes wrong during the running of the client.
    pub fn run(&mut self) -> Result<(), io::Error> {
        info!("Client Started");
        self.stream.write_all(&gen_client())?;

        let mut buf: ShakeBuf = new_shake_buf();

        self.stream.read_exact(&mut buf)?;

        if is_valid_server(buf) {
            info!("Connected to a Server");
        } else {
            warn!("Server failed handshake");
            self.stream.shutdown(Shutdown::Both)?;
            return Ok(());
        }
        
        info!("Disconnecting");
        self.stream.shutdown(Shutdown::Both)?;
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    CombinedLogger::init(vec![TermLogger::new(
        LevelFilter::Debug,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )])?;

    let address: SocketAddr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), PORT));

    let mut client: Client = match Client::try_new(address) {
        Ok(c) => c,
        Err(e) => match e.kind() {
            ErrorKind::ConnectionRefused => {
                error!("Connection Refused, Is the Server up?");
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
