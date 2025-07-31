use std::{ error::Error, net::{ Ipv4Addr, SocketAddrV4 }, sync::{ Arc, Mutex } };

use hecs::World;
use tokio::{ io::{ AsyncReadExt, AsyncWriteExt }, net::{ TcpListener, TcpStream }, spawn };

type Threadable<T> = Arc<Mutex<T>>; // std::sync::Mutex

const PORT: u16 = 9878;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let address: SocketAddrV4 = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), PORT);
    let listener: TcpListener = TcpListener::bind(address).await?;
    println!("Server bound to: {address}");

    World::new().query::<()>();

    loop {
        let (socket, _addr) = listener.accept().await?;
        spawn(async move {
            _ = process(socket).await;
        });
    }

    // Ok(())
}

/// Possibly push this into its own crate & port it to other langs
pub mod lore_proto {
    pub struct MessageHeader {
        code: MessageType,
        id_tag: u8,
        id: [u8; 6],
    }

    impl MessageHeader {
        pub fn to_bytes(self) -> [u8; 8] {
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

    pub fn create_id_tag(server: bool) -> u8 {
        let mut buf: u8 = 0u8;

        if server {
            buf += 1;
        }
        buf
    }

    /// Converts a byte into a message type
    pub fn u8_to_message(val: u8) -> Option<MessageType> {
        match val {
            0x00 => Some(MessageType::Connect),
            0x01 => Some(MessageType::Join),
            0x02 => Some(MessageType::Deny),
            0x03 => Some(MessageType::Quit),
            0x04 => Some(MessageType::Close),
            0x05 => Some(MessageType::Kick),
            0x06 => Some(MessageType::Drop),
            0x07 => Some(MessageType::GetEntity),
            0x08 => Some(MessageType::SendEntity),
            0x09 => Some(MessageType::GetPlayer),
            0x0A => Some(MessageType::SendPlayer),
            0x0B => Some(MessageType::GetComponents),
            0x0C => Some(MessageType::SendComponents),
            0x0D => Some(MessageType::GetRegisty),
            0x0E => Some(MessageType::SendRegistry),
            0x0F => Some(MessageType::GetRenderData),
            0x10 => Some(MessageType::SendRenderData),
            _ => None,
        }
    }

    /// Converts a message type into a byte
    pub fn message_to_u8(msg: MessageType) -> u8 {
        match msg {
            MessageType::Connect => 0x00,
            MessageType::Join => 0x01,
            MessageType::Deny => 0x02,
            MessageType::Quit => 0x03,
            MessageType::Close => 0x04,
            MessageType::Kick => 0x05,
            MessageType::Drop => 0x06,
            MessageType::GetEntity => 0x07,
            MessageType::SendEntity => 0x08,
            MessageType::GetPlayer => 0x09,
            MessageType::SendPlayer => 0x0A,
            MessageType::GetComponents => 0x0B,
            MessageType::SendComponents => 0x0C,
            MessageType::GetRegisty => 0x0D,
            MessageType::SendRegistry => 0x0E,
            MessageType::GetRenderData => 0x0F,
            MessageType::SendRenderData => 0x10,
        }
    }

    pub enum MessageType {
        Connect, // CONC
        Join, // JOIN
        Deny, // DENY
        Quit, // QUIT
        Close, // CLSO
        Kick, // KICK
        Drop, // DROP
        GetEntity, // GETE
        SendEntity, // RESE
        GetPlayer, // GETP
        SendPlayer, // RESP
        GetComponents, // GETC
        SendComponents, // RESC
        GetRegisty, // GREG
        SendRegistry, // SREG
        GetRenderData, // REND
        SendRenderData, // SHOW
    }
}

async fn process(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let mut buf = [0u8; 16];

    stream.read(&mut buf).await?;

    println!("{}", String::from_utf8_lossy(&buf[0..=7]));
    println!("{:?}", &buf[8..=15]);

    stream.write(b"Hello!:)").await?;

    Ok(())
}
