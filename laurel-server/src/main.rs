use std::{ error::Error, net::{ Ipv4Addr, SocketAddrV4 } };

use tokio::{ io::{ AsyncReadExt, AsyncWriteExt }, net::{ TcpListener, TcpStream }, spawn };

// type Data = Mutex<Arc<()>>; // std::sync::Mutex

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

async fn process(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let mut buf = [0u8; 4];

    stream.read(&mut buf).await?;
    println!("{:?}", String::from_utf8_lossy(&buf));
    buf.reverse();
    stream.write(&buf).await?;
    
    Ok(())
}
