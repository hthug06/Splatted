mod client;
mod errors;
mod packets;
mod server_info;
mod utils;

use crate::client::Client;
use crate::server_info::ServerInfo;
use clap::Parser;
use log::{LevelFilter, info};
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode};
use std::io::Error;

#[derive(Parser)]
#[command(version, about, long_about = None)]
/// The arguments passed when launching the program
pub struct Args {
    /// Address of the server
    #[arg(long)]
    address: String,

    /// Port of the server
    #[arg(short, default_value_t = 25565)]
    port: u16,

    /// Show info of the server (imitate the server list)
    #[arg(short, default_value_t = false)]
    info: bool,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Start log
    TermLogger::init(
        LevelFilter::Info,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )
    .unwrap();

    info!("Starting Splatted");

    // Get all the program arguments
    let args: Args = Args::parse();

    // parse the address (address:port)
    let address: String = format!("{}:{}", args.address, args.port);

    if args.info {
        ServerInfo::infos(&address).await.expect("Cannot ");
    } else {
        Client::new("player1").connect(address.as_str()).await?;
    }

    // Wait for a ctrl + C to finish, so the user can read info about the server for example
    tokio::signal::ctrl_c().await?;
    info!("Bye-bye !");

    Ok(())
}
