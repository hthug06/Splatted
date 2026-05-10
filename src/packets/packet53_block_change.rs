use crate::network::connection::Encryption;
use crate::packets::io::MinecraftReadExt;
use crate::packets::packet_trait::ServerPacket;
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
    ) -> Result<Self, Error>
    where
        Self: Sized,
    {
        Ok(Self {
            x: reader.read_i32(encryption).await?,
            y: reader.read_u8(encryption).await?,
            z: reader.read_i32(encryption).await?,
            block_id: reader.read_i16(encryption).await?,
            metadata: reader.read_u8(encryption).await?,
        })
    }
}
