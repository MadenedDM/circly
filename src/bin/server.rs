#![allow(clippy::multiple_crate_versions)]
use std::{
    error::Error,
    fmt::Debug,
    net::{Ipv4Addr, SocketAddr, SocketAddrV4},
    sync::{Arc, Mutex},
};

use common::api::{
    EchoRequest, EchoResponse,
    echo_server::{Echo, EchoServer},
};
use hecs::World;
use log::{LevelFilter, info};
use simplelog::{ColorChoice, CombinedLogger, Config, TermLogger, TerminalMode};

use tonic::{Request, Response, Status, transport::Server};

const PORT: u16 = 9878;

type Errorable = Result<(), Box<dyn Error>>;

/// A server implementation
pub struct ServerData {
    pub world: Arc<Mutex<World>>,
}

// impl Server {
//     /// Creates a server instance. Will immedately bind to a socket.
//     ///
//     /// Normally produces output to stdout
//     ///
//     /// # Panics
//     /// If the given socket can't be bound.
//     #[must_use]
//     pub async fn init(addr: SocketAddr) -> Self {
//         info!("Starting server .. {addr}");
//         let mut world: World = World::new();
//         world.spawn(());

//         let listen: TcpListener = match TcpListener::bind(addr).await {
//             Ok(lser) => {
//                 info!("Server bound ..... {addr}");
//                 lser
//             }
//             Err(e) => match e.kind() {
//                 ErrorKind::AddrInUse => {
//                     panic!("The socket to be bound is in use.")
//                 }
//                 _ => {
//                     panic!("Server Binding Failed")
//                 }
//             },
//         };

//         Self {
//             listener: listen,
//             world: Arc::new(Mutex::new(world)),
//             clients: Arc::new(Mutex::new(HashMap::new())),
//         }
//     }

//     /// Run the server
//     ///
//     /// # Errors
//     /// Sometimes
//     ///
//     /// # Panics
//     /// Sometimes
//     pub async fn run(&mut self) -> Errorable {
//         let server_id = gen_server();
//         loop {
//             match self.listener.accept().await {
//                 Ok((mut socket, address)) => {
//                     info!("New connection ... {address}");
//                     let mut handshake = new_shake_buf();
//                     let reading = socket.read_exact(&mut handshake);
//                     match reading.await {
//                         Ok(_amount) => {
//                             if is_valid_client(handshake) {
//                                 info!("Accepted ......... {address}");
//                                 match socket.write_all(&server_id).await {
//                                     Ok(()) => (),
//                                     Err(e) => {
//                                         error!("{e:?} Occured while confirming with client");
//                                         shutdown(&mut socket).await;
//                                     }
//                                 }
//                             } else {
//                                 info!("Refused .......... {address}");
//                                 shutdown(&mut socket).await;
//                             }
//                         }
//                         Err(e) => match e.kind() {
//                             ErrorKind::UnexpectedEof => {
//                                 info!("Failed Handshake . {address}");
//                                 shutdown(&mut socket).await;
//                             }
//                             _ => {
//                                 panic!("{e:?}");
//                             }
//                         },
//                     }
//                     spawn(async move {
//                         // let mut reader = BufReader::new(socket);
//                         // 'connection: loop {
//                         //     let mut buf = vec![0u8; 8];
//                         //     match reader.read_buf(&mut buf).await {
//                         //         Ok(qty) => {
//                         //             if qty == 0 {
//                         //                 println!("Disconnected ..... {address}");
//                         //                 break 'connection;
//                         //             }
//                         //             if buf != vec![0u8; 8] {
//                         //                 let j = String::from_utf8_lossy(&buf);

//                         //                 println!("{j:?} ... {address}");
//                         //             }
//                         //         }
//                         //         Err(e) => match e.kind() {
//                         //             ErrorKind::UnexpectedEof => {
//                         //                 println!("Disconnected ..... {address}");
//                         //                 break 'connection;
//                         //             }
//                         //             _ => {
//                         //                 println!("Error {e:?} while reading");
//                         //             }
//                         //         },
//                         //     }
//                         // }
//                     });
//                 }
//                 Err(e) => warn!("Failed connection: {e:?}"),
//             }
//         }
//     }
// }

#[derive(Debug, Default)]
pub struct EchoService {}

#[tonic::async_trait]
impl Echo for EchoService {
    async fn send(&self, request: Request<EchoRequest>) -> Result<Response<EchoResponse>, Status> {
        Ok(Response::new(EchoResponse {
            message: format!("hello {}", request.get_ref().name),
        }))
    }
}

// /// Handles shutting down a `TcpStream`
// ///
// /// # Panics
// /// If something goes wrong in the call to `TcpStream::shutdown`
// pub async fn shutdown(socket: &mut TcpStream) {
//     info!("Socket Closing");
//     match socket.shutdown().await {
//         Ok(()) => (),
//         Err(e) => {
//             // In case user is on MacOS, this might change in the future
//             assert!((e.kind() == ErrorKind::NotConnected), "Error {e:?}");
//         }
//     }
// }

#[tokio::main]
async fn main() -> Errorable {
    CombinedLogger::init(vec![TermLogger::new(
        LevelFilter::Debug,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )])
    .unwrap();

    info!("Starting Server");

    let address: SocketAddr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), PORT));

    Server::builder()
        .add_service(EchoServer::new(EchoService::default()))
        .serve(address)
        .await?;

    info!("Server Closed");

    Ok(())
}
