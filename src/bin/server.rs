#![allow(clippy::multiple_crate_versions)]
use std::{
    error::Error,
    fmt::Debug,
    net::{Ipv4Addr, SocketAddr, SocketAddrV4},
};

use common::api::{
    EchoRequest, EchoResponse, JoinRequest, JoinResponse, QuitRequest, QuitResponse,
    main_server::{Main, MainServer},
};
#[cfg(feature = "log")]
use log::{LevelFilter, debug, info};
#[cfg(feature = "log")]
use simplelog::{ColorChoice, CombinedLogger, Config, TermLogger, TerminalMode};

use tonic::{Request, Response, Status, transport::Server};

const PORT: u16 = 9878;

type Errorable = Result<(), Box<dyn Error>>;

#[derive(Debug)]
pub struct Service {}

#[tonic::async_trait]
impl Main for Service {
    async fn join(&self, request: Request<JoinRequest>) -> Result<Response<JoinResponse>, Status> {
        let inner = request.get_ref();

        #[cfg(feature = "log")]
        info!("{} Has Joined!", inner.name);
        Ok(Response::new(JoinResponse { id: 1 }))
    }

    async fn quit(&self, request: Request<QuitRequest>) -> Result<Response<QuitResponse>, Status> {
        let inner = request.get_ref();

        #[cfg(feature = "log")]
        info!("{} [{}] Is Quitting!", inner.name, inner.id);

        Ok(Response::new(QuitResponse { exitcode: 0 }))
    }

    async fn echo(&self, request: Request<EchoRequest>) -> Result<Response<EchoResponse>, Status> {
        let inner = request.get_ref();

        #[cfg(feature = "log")]
        debug!("{inner:?}");

        Ok(Response::new(EchoResponse {
            dat: inner.dat.clone(),
        }))
    }
}

#[tokio::main]
async fn main() -> Errorable {
    #[cfg(feature = "log")]
    CombinedLogger::init(vec![TermLogger::new(
        LevelFilter::Debug,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )])
    .unwrap();

    #[cfg(feature = "log")]
    info!("Starting Server");

    let address: SocketAddr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::LOCALHOST, PORT));

    #[cfg(feature = "log")]
    info!("Using address: {address}");

    let service = Service {};

    Server::builder()
        .add_service(MainServer::new(service))
        .serve(address)
        .await?;

    #[cfg(feature = "log")]
    info!("Server Closed");

    Ok(())
}
