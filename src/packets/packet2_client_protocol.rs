use crate::network::connection::Encryption;
use crate::packets::io::{MinecraftReadExt, MinecraftWriteExt};
use crate::packets::packet_trait::{ClientPacket, ServerPacket};
use crate::protocol_version::ProtocolVersion;
use bytes::{BufMut, BytesMut};
use std::io::Error;
use tokio::io::BufReader;
use tokio::net::tcp::OwnedReadHalf;

pub struct ClientHandshakePacket {
    /// This might change if we try to support more version ( > 1.10.2 is u16)
    pub protocol_version: u8,
    pub username: String,
    pub server_hostname: String,
    pub server_port: u32,
}

impl ClientHandshakePacket {
    pub fn new(
        protocol_version: u8,
        username: &str,
        server_hostname: String,
        server_port: u32,
    ) -> Self {
        Self {
            protocol_version,
            username: username.to_string(),
            server_hostname,
            server_port,
        }
    }
}

impl ClientPacket for ClientHandshakePacket {
    fn write_to(
        &self,
        buffer: &mut BytesMut,
        protocol_version: ProtocolVersion,
    ) -> Result<(), Error> {
        //DON'T FORGET TO ADD THE PACKET ID
        buffer.put_u8(0x02);

        // Add all the infos
        // Modify for future version
        match protocol_version {
            ProtocolVersion::V1_3
            | ProtocolVersion::V1_4
            | ProtocolVersion::V1_5
            | ProtocolVersion::V1_6 => {
                buffer.put_u8(self.protocol_version);
                buffer.write_string(&self.username)?;
                buffer.write_string(&self.server_hostname)?;
                buffer.put_u32(self.server_port);
            }
            _ => {
                // 1.2 server only want 1 big string
                let combined_string = format!(
                    "{};{}:{}",
                    self.username, self.server_hostname, self.server_port
                );
                buffer.write_string(&combined_string)?;
            }
        }

        Ok(())
    }
}

pub struct ServerHandshakePacket {
    /// The response to this packet is:
    /// - Online mode: key to encrypt the connexion with the mojang API
    /// - Offline mode (us): char "-"
    pub connection_hash: String,
}

impl ServerPacket for ServerHandshakePacket {
    async fn read(
        reader: &mut BufReader<OwnedReadHalf>,
        encryption: &mut Encryption,
        _protocol_version: ProtocolVersion,
    ) -> Result<Self, Error>
    where
        Self: Sized,
    {
        Ok(Self {
            connection_hash: reader.read_string(encryption).await?,
        })
    }
}
