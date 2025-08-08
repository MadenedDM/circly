use std::{
    collections::HashMap,
    error::Error,
    io::ErrorKind,
    net::{Ipv4Addr, SocketAddr, SocketAddrV4},
    sync::{Arc, Mutex},
};

use hecs::World;
use tokio::{
    net::{TcpListener, TcpStream},
    spawn,
};

const PORT: u16 = 9878;

type Errorable = Result<(), Box<dyn Error>>;

// #[derive(Clone, Copy, Debug)]
// struct PlayerComponent {
//     owner: u32,
// }

// struct Position {
//     x: i32,
//     y: i32,
// }

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    #[must_use]
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

pub struct Server {
    pub listener: TcpListener,

    pub clients: Arc<Mutex<HashMap<SocketAddr, TcpStream>>>,
    pub world: Arc<Mutex<World>>,
}

impl Server {
    /// Creates a server instance. Will immedately bind to a socket.
    ///
    /// Normally produces output to stdout
    ///
    /// # Panics
    /// If the given socket can't be bound.
    #[must_use]
    pub async fn init(addr: SocketAddr) -> Self {
        println!("Starting server on: {addr}");
        let mut world: World = World::new();
        world.spawn(());

        let listen: TcpListener = match TcpListener::bind(addr).await {
            Ok(lser) => {
                println!("Server bound to: {addr}");
                lser
            }
            Err(e) => match e.kind() {
                ErrorKind::AddrInUse => {
                    panic!("The socket to be bound is in use.")
                }
                _ => {
                    panic!("Server Binding Failed")
                }
            },
        };

        Self {
            listener: listen,
            world: Arc::new(Mutex::new(world)),
            clients: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Run the server
    ///
    /// # Errors
    /// Sometimes
    ///
    /// # Panics
    /// Sometimes
    pub async fn run(&mut self) -> Errorable {
        loop {
            match self.listener.accept().await {
                Ok((socket, address)) => {
                    println!("New connection {address:?}");
                    spawn(async move {
                        println!("{:?}", socket.readable().await.unwrap());
                    });
                }
                Err(e) => println!("Failed connection: {e:?}"),
            }
        }
    }
}

#[tokio::main]
async fn main() -> Errorable {
    let address: SocketAddr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), PORT));

    let mut server: Server = Server::init(address).await;

    server.run().await?;

    Ok(())
}
