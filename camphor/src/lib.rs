use laurel_common::camphor::Request;
use rkyv::{from_bytes, rancor::Error, to_bytes};
use std::{
    io::{Read, Write},
    net::TcpStream,
};


/// Sends a request from a socket
///
/// # Errors
/// If the `to_bytes` method errors, the writes will not produce errors though
///
/// # Panics
/// If the `write_all` method errors.
pub fn request(request: &Request, socket: &mut TcpStream) -> Result<(), Error> {
    match to_bytes::<Error>(request).inspect(|ser| {
        socket
            .write_all(ser.as_slice())
            .expect("Failed to send Request");
    }) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

/// Recieves a request, returns None if the data is invalid or if something goes wrong reading from the socket.
pub fn recieve_request(socket: &mut TcpStream) -> Option<Request> {
    let mut vbuf: Vec<u8> = vec![];
    loop {
        let mut buf: [u8; 16] = [0u8; 16];
        match socket.read(&mut buf) {
            Ok(0) => {
                break;
            }
            Err(_) => {
                return None;
            }
            Ok(n) => {
                vbuf.extend_from_slice(&buf[0..=n]);
            }
        }
    }
    from_bytes::<Request, Error>(vbuf.as_slice()).ok()
}
