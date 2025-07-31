use std::{ error::Error, io::{ stdin, Read }, net::{ Ipv4Addr, SocketAddrV4 } };

use tokio::{ io::{ AsyncReadExt, AsyncWriteExt }, net::TcpStream };

const PORT: u16 = 9878;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let address: SocketAddrV4 = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), PORT);
    let mut stream = TcpStream::connect(address).await?;
    println!("Listening on: {address}");

    let mut sin = stdin();

    let mut bufw = [0u8; 8];
    _ = sin.read(&mut bufw);

    stream.write(&bufw).await?;

    let mut buf = [0u8; 8];
    stream.read(&mut buf).await?;

    println!("{:?}", String::from_utf8_lossy(&buf));

    Ok(())
}
