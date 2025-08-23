#![allow(clippy::multiple_crate_versions)]
use std::error::Error;

use common::api::{EchoRequest, JoinRequest, QuitRequest, main_client::MainClient};
#[cfg(feature = "log")]
use log::{LevelFilter, info};
#[cfg(feature = "log")]
use simplelog::{ColorChoice, CombinedLogger, Config, TermLogger, TerminalMode};
use tonic::{Request, transport::Channel};

const PORT: u16 = 9878;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    #[cfg(feature = "log")]
    CombinedLogger::init(vec![TermLogger::new(
        LevelFilter::Debug,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )])?;

    #[cfg(feature = "log")]
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

    let d = client
        .echo(Request::new(EchoRequest {
            id,
            dat: "Hello!".into(),
        }))
        .await?;

    #[cfg(feature = "log")]
    info!("{}", d.into_inner().dat);

    client
        .quit(Request::new(QuitRequest {
            name: "Bob".into(),
            id,
        }))
        .await?;

    #[cfg(feature = "log")]
    info!("Ending Client");

    Ok(())
}
