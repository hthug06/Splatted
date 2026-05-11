use crate::network::connection::Encryption;
use crate::packets::io::MinecraftReadExt;
use crate::packets::packet_trait::ServerPacket;
use crate::protocol_version::ProtocolVersion;
use std::io::Error;
use tokio::io::BufReader;
use tokio::net::tcp::OwnedReadHalf;

pub struct PlayerInfoPacket {
    pub name: String,
    pub is_connected: bool,
    pub ping: i16,
}

impl ServerPacket for PlayerInfoPacket {
    async fn read(
        reader: &mut BufReader<OwnedReadHalf>,
        encryption: &mut Encryption,
        _protocol_version: ProtocolVersion,
    ) -> Result<Self, Error>
    where
        Self: Sized,
    {
        Ok(Self {
            name: reader.read_string(encryption).await?,
            is_connected: reader.read_u8(encryption).await? != 0,
            ping: reader.read_i16(encryption).await?,
        })
    }
}
