use crate::network::connection::Encryption;
use crate::packets::packet_trait::ServerPacket;
use crate::packets::packet30_entity::EntityPacket;
use crate::packets::types::destroy_stage::DestroyStage;
use crate::packets::utils::{read_i32, read_u8};
use std::io::Error;
use tokio::io::BufReader;
use tokio::net::tcp::OwnedReadHalf;

pub struct BlockDestroyPacket {
    pub entity: EntityPacket,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub destroyed_stage: DestroyStage,
}

impl ServerPacket for BlockDestroyPacket {
    async fn read(
        reader: &mut BufReader<OwnedReadHalf>,
        encryption: &mut Encryption,
    ) -> Result<Self, Error>
    where
        Self: Sized,
    {
        Ok(Self {
            entity: EntityPacket::read(reader, encryption).await?,
            x: read_i32(reader, encryption).await?,
            y: read_i32(reader, encryption).await?,
            z: read_i32(reader, encryption).await?,
            destroyed_stage: DestroyStage::from_id(read_u8(reader, encryption).await?),
        })
    }
}
