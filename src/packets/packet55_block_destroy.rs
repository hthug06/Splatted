use crate::network::connection::Encryption;
use crate::packets::io::MinecraftReadExt;
use crate::packets::packet_trait::ServerPacket;
use crate::packets::packet30_entity::EntityPacket;
use crate::packets::types::destroy_stage::DestroyStage;
use crate::protocol_version::ProtocolVersion;
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
        protocol_version: ProtocolVersion,
    ) -> Result<Self, Error>
    where
        Self: Sized,
    {
        Ok(Self {
            entity: EntityPacket::read(reader, encryption, protocol_version).await?,
            x: reader.read_i32(encryption).await?,
            y: reader.read_i32(encryption).await?,
            z: reader.read_i32(encryption).await?,
            destroyed_stage: DestroyStage::from_id(reader.read_u8(encryption).await?),
        })
    }
}
