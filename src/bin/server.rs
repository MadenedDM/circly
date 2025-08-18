#![allow(clippy::multiple_crate_versions)]
use std::{
    error::Error,
    fmt::Debug,
    net::{Ipv4Addr, SocketAddr, SocketAddrV4},
    sync::{Arc, Mutex},
};

use common::api::{
    talk_server::{Talk, TalkServer}, EchoRequest, EchoResponse, GreetRequest, GreetResponse
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
pub struct TalkService {}

#[tonic::async_trait]
impl Talk for TalkService {
    async fn echo(&self, request: Request<EchoRequest>) -> Result<Response<EchoResponse>, Status> {
        let inner = request.get_ref();
        debug!("{inner:?}");
        Ok(Response::new(EchoResponse {
            message: inner.message.to_string(),
        }))
    }
    async fn greet(&self, request: Request<GreetRequest>) -> Result<Response<GreetResponse>, Status> {
        let inner = request.get_ref();
        debug!("{inner:?}");
        Ok(Response::new(GreetResponse {
            message: format!("Hello {}!", inner.name),
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

    info!("Using address: {address}");

    Server::builder()
        .add_service(TalkServer::new(TalkService::default()))
        .serve(address)
        .await?;

    info!("Server Closed");

    Ok(())
}
