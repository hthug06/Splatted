use crate::network::connection::Encryption;
use crate::packets::packet_trait::ServerPacket;
use crate::packets::utils::{read_i16, read_i32, read_u8};
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
            x: read_i32(reader, encryption).await?,
            y: read_u8(reader, encryption).await?,
            z: read_i32(reader, encryption).await?,
            block_id: read_i16(reader, encryption).await?,
            metadata: read_u8(reader, encryption).await?,
        })
    }
}
