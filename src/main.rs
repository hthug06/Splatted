use clap::Parser;
use log::{LevelFilter, info};
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
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
}

#[tokio::main]
async fn main() {
    // Start log
    TermLogger::init(
        LevelFilter::Info,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )
    .unwrap();

    info!("starting splatted");

    // Get all the program arguments
    let args: Args = Args::parse();

    // parse the address (address:port)
    let address: String = format!("{}:{}", args.address, args.port);

    if args.info {
        // Connect to the server
        let mut stream: TcpStream = TcpStream::connect(address).await.unwrap();

        // Send first packet
        stream.write_all(&[254, 1]).await.unwrap();

        //Create a buffer
        // in this version, we can't know before the size of the buffer
        // 1024 might be good for now, but we might need to extend it when parsing chunk packet...
        // /** A temporary storage for the compressed chunk data byte array. */
        // private static byte[] temp = new byte[196864]; in source code so yeeeee...
        let mut buffer: [u8; 1024] = [0; 1024];

        // Listen for packet
        loop {
            let bytes_read = stream.read(&mut buffer).await.unwrap();

            // For tests only (don't want to pollute wireshark ;) )
            if bytes_read == 0 {
                println!("Connection closed.");
                break;
            }

            // Treat packet here (with match for example)
            let received_data: &[u8] = &buffer[..bytes_read];
            info!("Received : {:?}", &received_data[1..]);
            info!(
                "Received + translated: {:?}",
                read_utf16_from_buffer(&received_data[1..])
            );
        }
    } else {
        unimplemented!("For now, only the info mode can be used")
    }

    // Wait for a ctrl + C to finish, so the user can read info about the server for example
    tokio::signal::ctrl_c().await.unwrap();
    info!("Bye-bye !");
}

fn read_utf16_from_buffer(buffer: &[u8]) -> String {
    let mut utf16buffer: Vec<u16> = vec![];
    let mut elt1: Option<u8> = None;
    for i in 0..buffer.len() {
        // First part of u16, just store if
        if i % 2 == 0 {
            elt1 = Some(buffer[i]);
        }
        //Second part, shift and create the u16
        else {
            utf16buffer.push(u16::from_be_bytes([elt1.unwrap(), buffer[i]]));

            //reset
            elt1 = None;
        }
    }

    if let Some(final_elt) = elt1 {
        utf16buffer.push(final_elt as u16)
    }

    // Let the \0 live, so we can parse the text if this use more than 1 variable
    // Like for the server info, this return : #§1\051\01.4.7\0A Minecraft Server\01\020
    // Here we can clearly see the protocol, MOTD, player count and max player
    String::from_utf16_lossy(&utf16buffer)
}
