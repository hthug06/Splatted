use crate::errors::wrong_packet_error::WrongPacketError;
use crate::network::connection::Encryption;
use crate::packets::packet_trait::{ClientPacket, ServerPacket};
use crate::packets::packet254_server_ping::ServerPingPacket;
use crate::packets::packet255_kick_disconnect::KickDisconnectPacket;
use crate::packets::utils::read_u8;
use std::io::{Error, ErrorKind};
use tokio::io::{AsyncWriteExt, BufReader};
use tokio::net::TcpStream;

pub struct ServerInfo;

impl ServerInfo {
    pub async fn infos(address: &String) -> Result<(), Error> {
        // Connect to the server
        let stream = TcpStream::connect(address).await?;

        // split the stream + create the reader
        let (read_half, mut write_half) = stream.into_split();
        let mut reader = BufReader::new(read_half);

        // When we ping the server, nothing is encrypted. But we still need a base encryption
        let mut encryption = Encryption::new();

        // Send first packet (Server Ping = 0xFE)
        let mut buffer: Vec<u8> = vec![];
        ServerPingPacket.write_to(&mut buffer)?;
        write_half.write_all(&buffer).await?;
        write_half.flush().await?;

        // Listen for the response
        let packet_id = read_u8(&mut reader, &mut encryption).await?;

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
            KickDisconnectPacket::read(&mut reader, &mut encryption).await?;

        // Print all the infos
        log::info!("{}", kick_disconnect_packet.format_server_infos());

        // After receiving the serverListPacket, the connection close
        write_half.shutdown().await?;

        drop(write_half);
        drop(reader);

        log::info!("Connection closed.");
        Ok(())
    }
}
