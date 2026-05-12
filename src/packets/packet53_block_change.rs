use crate::network::connection::Encryption;
use crate::packets::io::MinecraftReadExt;
use crate::packets::packet_trait::ServerPacket;
use crate::protocol_version::ProtocolVersion;
use std::io::Error;
use tokio::io::BufReader;
use tokio::net::tcp::OwnedReadHalf;

pub struct BlockChangePacket {
    pub x: i32,
    pub y: u8,
    pub z: i32,
    pub block_id: i16,
    pub metadata: u8,
}

impl ServerPacket for BlockChangePacket {
    async fn read(
        reader: &mut BufReader<OwnedReadHalf>,
        encryption: &mut Encryption,
        protocol_version: ProtocolVersion,
    ) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let x = reader.read_i32(encryption).await?;
        let y = reader.read_u8(encryption).await?;
        let z = reader.read_i32(encryption).await?;

        // 1.2 don't have that much block so it's an u8
        let block_id = if protocol_version == ProtocolVersion::V1_2 {
            reader.read_u8(encryption).await? as i16
        }
        // From 1.3, we can't read u8 anymore because there is more blocks, so it's an i16
        else {
            reader.read_i16(encryption).await?
        };
        let metadata = reader.read_u8(encryption).await?;

        Ok(Self {
            x,
            y,
            z,
            block_id,
            metadata,
        })
    }
}
