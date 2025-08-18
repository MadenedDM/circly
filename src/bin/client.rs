#![allow(clippy::multiple_crate_versions)]
use std::error::Error;

use common::api::{main_client::MainClient, EchoRequest, JoinRequest, QuitRequest};
use log::{LevelFilter, info};
use simplelog::{ColorChoice, CombinedLogger, Config, TermLogger, TerminalMode};
use tonic::{Request, transport::Channel};

const PORT: u16 = 9878;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    CombinedLogger::init(vec![TermLogger::new(
        LevelFilter::Debug,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )])?;

    info!("Starting Client");

    let channel: Channel = Channel::from_shared(format!("http://localhost:{}", PORT))?
        .connect()
        .await?;

    let mut client = MainClient::new(channel);

    let id = client
        .join(Request::new(JoinRequest { name: "Bob".into() }))
        .await?
        .into_inner()
        .id;

    let d = client.echo(Request::new(EchoRequest {id, dat: "Hello!".into()})).await?;

    info!("{}", d.into_inner().dat);

    client
        .quit(Request::new(QuitRequest {
            name: "Bob".into(),
            id,
        }))
        .await?;

    info!("Ending Client");

    Ok(())
}
