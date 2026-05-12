use crate::network::connection::Encryption;
use crate::packets::io::MinecraftReadExt;
use crate::packets::packet_trait::ServerPacket;
use crate::protocol_version::ProtocolVersion;
use std::io::Error;
use tokio::io::BufReader;
use tokio::net::tcp::OwnedReadHalf;

/// Only in 1.2
pub struct PreChunkPacket {
    pub x: i32,
    /// In the mc code, it's called "y", but z is more logical here
    pub z: i32,
    pub mode: bool,
}

impl ServerPacket for PreChunkPacket {
    async fn read(
        reader: &mut BufReader<OwnedReadHalf>,
        encryption: &mut Encryption,
        _protocol_version: ProtocolVersion,
    ) -> Result<Self, Error>
    where
        Self: Sized,
    {
        Ok(Self {
            x: reader.read_i32(encryption).await?,
            z: reader.read_i32(encryption).await?,
            mode: reader.read_u8(encryption).await? != 0,
        })
    }
}
