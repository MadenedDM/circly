pub use rkyv;

#[cfg(feature = "tokio")]
use tokio::{io::AsyncWriteExt, net::TcpStream};

#[cfg(not(feature = "tokio"))]
use std::{io::Write, net::TcpStream};

use rkyv::{Archive, Deserialize, Serialize, rancor, to_bytes};

pub type ShakeBuf = [u8; 16];

pub const EOF: u8 = 0x03;
const CLIENT: ShakeBuf = [0x21; 16]; // ! in ascii
const SERVER: ShakeBuf = [0xA1; 16]; // ¡ in ascii

#[must_use]
pub const fn new_shake_buf() -> ShakeBuf {
    [0u8; 16]
}

#[derive(Debug, Clone, Serialize, Deserialize, Archive)]
#[rkyv(compare(PartialEq), derive(Debug))]
#[non_exhaustive]
pub enum Request {
    Echo(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, Archive)]
#[rkyv(compare(PartialEq), derive(Debug))]
#[non_exhaustive]
pub enum Response {
    Fail,
    Refuse,
    Data(String),
}

#[cfg(not(feature = "tokio"))]
/// Sends a request from a socket
///
/// # Errors
/// If the `to_bytes` method errors, the writes will not produce errors though
///
/// # Panics
/// If the `write_all` method errors.
pub fn send(request: &Request, socket: &mut TcpStream) -> Result<(), rancor::Error> {
    match to_bytes::<rancor::Error>(request).inspect(|ser| {
        socket
            .write_all(ser.as_slice())
            .expect("Failed to send Request");
        socket.write_all(&[EOF]).expect("Failed to end Request");
    }) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

#[cfg(feature = "tokio")]
/// Sends a request from a socket
///
/// # Errors
/// If the `to_bytes` method errors, the writes will not produce errors though
///
/// # Panics
/// If the `write_all` method errors.
pub async fn send(request: &Request, socket: &mut TcpStream) -> Result<(), rancor::Error> {
    match to_bytes::<rancor::Error>(request) {
        Ok(ser) => {
            socket
                .write_all(ser.as_slice())
                .await
                .expect("Failed to send Request");
            socket
                .write_all(&[EOF])
                .await
                .expect("Failed to end Request");
            Ok(())
        }
        Err(e) => Err(e),
    }
}

#[must_use]
pub fn is_valid_server(dat: ShakeBuf) -> bool {
    dat == SERVER
}

#[must_use]
pub fn is_valid_client(dat: ShakeBuf) -> bool {
    dat == CLIENT
}

#[must_use]
pub fn gen_client() -> ShakeBuf {
    CLIENT
}

#[must_use]
pub fn gen_server() -> ShakeBuf {
    SERVER
}

#[cfg(test)]
mod test {
    use crate::lore::{gen_client, gen_server, is_valid_client, is_valid_server};

    #[test]
    fn test_client_dat() {
        assert!(is_valid_client(gen_client()));
    }

    #[test]
    fn test_server_dat() {
        assert!(is_valid_server(gen_server()));
    }
}
