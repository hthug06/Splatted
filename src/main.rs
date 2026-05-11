mod client;
mod errors;
mod network;
mod packets;
mod protocol_version;
mod server_info;

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

    #[arg(long, short, default_value_t = 10)]
    bot_number: u32,

    #[arg(long, short = 'r', default_value_t = 51)]
    protocol: u8,
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
    .expect("Failed to initialize logger");

    info!("Starting Splatted");

    // Get all the program arguments
    let args: Args = Args::parse();

    // parse the address (address:port)
    let address: String = format!("{}:{}", args.address, args.port);

    if args.info {
        ServerInfo::infos(&address).await?;
    } else {
        // Create the bot task list to keep the connection active
        let mut bot_tasks = vec![];

        for i in 0..args.bot_number {
            info!("Connecting bot {}", i);

            // Need to clone the  address because it's going to be in the async move
            let address_clone = address.clone();

            // Launch the tasks
            let task = tokio::spawn(async move {
                let bot_name = format!("player{}", i);
                let mut client = Client::new(bot_name.as_str(), args.protocol);

                // Await here to do async and not block the program here
                if let Err(e) = client.connect(address_clone.as_str()).await {
                    log::error!("bot {} crashed : {}", bot_name, e);
                }
            });

            bot_tasks.push(task);
        }

        for task in bot_tasks {
            let _ = task.await;
        }
    }

    // Wait for a ctrl + C to finish, so the user can read info about the server for example
    tokio::signal::ctrl_c().await?;
    info!("Bye-bye !");
    Ok(())
}
