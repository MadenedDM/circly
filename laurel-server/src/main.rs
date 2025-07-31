use std::{ array, error::Error, net::{ Ipv4Addr, SocketAddrV4 }, sync::{ Arc, Mutex } };

use tokio::{ io::{ AsyncReadExt, AsyncWriteExt }, net::{ TcpListener, TcpStream }, spawn };

type Threadable<T> = Arc<Mutex<T>>; // std::sync::Mutex

const PORT: u16 = 9878;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let address: SocketAddrV4 = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), PORT);
    let listener: TcpListener = TcpListener::bind(address).await?;
    println!("Server bound to: {address}");

    loop {
        let (socket, _addr) = listener.accept().await?;
        spawn(async move {
            _ = process(socket).await;
        });
    }

    // Ok(())
}

struct MessageHeader {
    code: MessageType,
    id_tag: u8,
    id: [u8; 6],
}

impl MessageHeader {
    fn to_bytes(self) -> [u8; 8] {
        [
            message_to_u8(self.code),
            self.id_tag,
            self.id[0],
            self.id[1],
            self.id[2],
            self.id[3],
            self.id[4],
            self.id[5],
        ]
    }
}

fn create_id_tag(server: bool) -> u8 {
    let mut buf: u8 = 0u8;

    if server {
        buf += 1;
    }
    buf
}

/// Converts a byte into a message type
fn u8_to_message(val: u8) -> Option<MessageType> {
    match val {
        0x00 => Some(MessageType::Connect),
        _ => None,
    }
}

/// Converts a message type into a byte
fn message_to_u8(msg: MessageType) -> u8 {
    match msg {
        MessageType::Connect => 0x00,
        MessageType::Join => 0x01,
        MessageType::Deny => 0x02,
        MessageType::Quit => 0x03,
        MessageType::Close => 0x04,
        MessageType::Kick => 0x05,
        MessageType::Drop => 0x06,
    }
}

enum MessageType {
    Connect,
    Join,
    Deny,
    Quit,
    Close,
    Kick,
    Drop,
}

async fn process(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let mut buf = [0u8; 16];

    stream.read(&mut buf).await?;

    println!("{}", String::from_utf8_lossy(&buf[0..=7]));
    println!("{:?}", &buf[8..=15]);

    stream.write(b"Hello!:)").await?;

    Ok(())
}
