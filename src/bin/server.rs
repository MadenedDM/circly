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
use log::{debug, info, LevelFilter};
use simplelog::{ColorChoice, CombinedLogger, Config, TermLogger, TerminalMode};

use tonic::{Request, Response, Status, transport::Server};

const PORT: u16 = 9878;

type Errorable = Result<(), Box<dyn Error>>;

/// A server implementation
pub struct ServerData {
    pub world: Arc<Mutex<World>>,
}

#[derive(Debug, Default)]
pub struct EchoService {}

#[tonic::async_trait]
impl Echo for EchoService {
    async fn send(&self, request: Request<EchoRequest>) -> Result<Response<EchoResponse>, Status> {
        debug!("{request:?}");
        Ok(Response::new(EchoResponse {
            message: request.get_ref().name.to_string(),
        }))
    }
}

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
