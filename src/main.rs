mod client;
mod errors;
mod network;
mod packets;
mod protocol_version;
mod server_info;

use crate::client::Client;
use crate::errors::wrong_packet_error::WrongPacketError;
use crate::network::connection::Encryption;
use crate::packets::io::MinecraftReadExt;
use crate::packets::packet_trait::{ClientPacket, ServerPacket};
use crate::packets::packet254_server_ping::ServerPingPacket;
use crate::packets::packet255_kick_disconnect::{KickDisconnectPacket, ServerPingResponse};
use crate::protocol_version::ProtocolVersion;
use crate::server_info::ServerInfo;
use bytes::BytesMut;
use clap::Parser;
use log::{LevelFilter, info};
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode};
use std::io::{Error, ErrorKind};
use tokio::io::{AsyncWriteExt, BufReader};
use tokio::net::TcpStream;

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

    #[arg(long, short, default_value_t = 1)]
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

    info!("Starting Splatted v{}", env!("CARGO_PKG_VERSION"));

    // Get all the program arguments
    // mut because the protocol version can change later
    let mut args: Args = Args::parse();

    // parse the address (address:port)
    let address: String = format!("{}:{}", args.address, args.port);
    info!("Connecting {} bot(s) to {}", args.bot_number, address);

    // Get the info like in the server list
    if args.info {
        ServerInfo::infos(&address).await?;
    }
    // Connect bots
    else {
        // Get the real protocol version
        let exact_protocol_version = get_real_protocol_version(&mut args, &address).await?;

        //Check if version is supported
        ProtocolVersion::from_protocol_version(exact_protocol_version as u32)?;

        // Create the bot task list to keep the connection active
        let mut bot_tasks = vec![];

        for i in 0..args.bot_number {
            info!("Connecting bot {}", i + 1);

            // Need to clone the address because it's going to be in the async move
            let address_clone = address.clone();

            // Launch the tasks
            let task = tokio::spawn(async move {
                let bot_name = format!("player{}", i + 1);
                let mut client = Client::new(bot_name.as_str(), exact_protocol_version as u8);

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

async fn get_real_protocol_version(args: &mut Args, address: &str) -> Result<u16, Error> {
    let stream = TcpStream::connect(address).await?;
    stream.set_nodelay(true)?;

    // Split the TCP stream into owned read and write halves.
    let (stream_reader, mut writer) = stream.into_split();
    let mut reader = BufReader::with_capacity(1024 * 64, stream_reader);

    // Send first packet (Server Ping = 0xFE)
    let mut buffer = BytesMut::new();
    // The protocol version can be anything because we it doesn't change anything in this packet
    ServerPingPacket.write_to(&mut buffer, ProtocolVersion::V1_4)?;
    writer.write_all(&buffer).await?;
    writer.flush().await?;

    // Listen for the response
    let packet_id = MinecraftReadExt::read_u8(&mut reader, &mut Encryption::new()).await?;

    // check if we received the right packet
    if packet_id != 255 {
        return Err(Error::new(
            ErrorKind::InvalidData,
            format!(
                "{}",
                WrongPacketError {
                    attended: 255,
                    received: packet_id
                }
            ),
        ));
    }

    let kick_disconnect_packet =
        KickDisconnectPacket::read(&mut reader, &mut Encryption::new(), ProtocolVersion::V1_4)
            .await?;

    let exact_protocol_version = if let Some(protocol) =
        ServerPingResponse::from_kickdisconnect(&kick_disconnect_packet)?.protocol
    {
        if protocol != args.protocol as u16 {
            log::warn!(
                "Ignore this message if you didn't set the server protocol version in the CLI arguments."
            );
            log::warn!(
                "Found real server protocol version : {}. You specified : {}. Now using server protocol: {}",
                protocol,
                args.protocol,
                protocol
            );
        }
        protocol
    } else {
        args.protocol as u16
    };
    Ok(exact_protocol_version)
}
