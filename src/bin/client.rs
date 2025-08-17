#![allow(clippy::multiple_crate_versions)]
use std::error::Error;

use common::api::{talk_client::TalkClient, EchoRequest, GreetRequest};
use log::{LevelFilter, debug, info};
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

    let mut client = TalkClient::new(channel);

    {
        let request = Request::new(GreetRequest {
            name: String::from("Bob"),
        });

        let response = client.greet(request).await?;
        debug!("{:?}", response.metadata());
        info!("{:?}", response.get_ref());
    }

    loop {
        let request = Request::new(EchoRequest {
            message: String::from("Goodbye!"),
        });

        let response = client.echo(request).await?;
        debug!("{:?}", response.metadata());
        info!("{:?}", response.get_ref());
    }

    info!("Ending Client");

    Ok(())
}
