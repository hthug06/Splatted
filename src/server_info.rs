use crate::errors::wrong_packet_error::WrongPacketError;
use crate::packets::ClientPacket;
use crate::packets::ServerPacket;
use crate::packets::packet254_server_ping::ServerPing;
use crate::packets::packet255_kick_disconnect::KickDisconnect;
use std::io::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

pub struct ServerInfo;

impl ServerInfo {
    pub async fn infos(address: &String) -> Result<(), Error> {
        // Connect to the server
        let mut stream: TcpStream = TcpStream::connect(address).await?;

        // Send first packet
        stream.write_all(&ServerPing::write()).await?;

        //Create a buffer
        // in this version, we can't know before the size of the buffer
        // 1024 might be good for now, but we might need to extend it when parsing chunk packet...
        // /** A temporary storage for the compressed chunk data byte array. */
        // private static byte[] temp = new byte[196864]; in source code so yeeeee...
        let mut buffer: [u8; 1024] = [0; 1024];

        // Listen for packet
        loop {
            let bytes_read = stream.read(&mut buffer).await?;
            // We only can get the server infos here
            // But if we get another packet, we throw an error
            let received_data: &[u8] = &buffer[..bytes_read];

            // Check if the right packet is received
            let kick_disconnect_packet = match received_data[0] {
                255 => Ok(KickDisconnect::read(received_data)),
                _ => Err(format!(
                    "{}",
                    WrongPacketError {
                        attended: 255,
                        received: received_data[0]
                    }
                )),
            }
            .unwrap();

            // Print all the infos
            log::info!("{}", kick_disconnect_packet.format_server_infos());

            // After receiving the serverListPacket, the connection close and we stop listening to packet
            stream.shutdown().await?;
            log::info!("Connection closed.");
            break;
        }

        Ok(())
    }
}
