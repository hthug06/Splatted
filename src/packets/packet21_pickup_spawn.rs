use crate::network::connection::Encryption;
use crate::packets::io::MinecraftReadExt;
use crate::packets::packet_trait::ServerPacket;
use crate::packets::packet30_entity::EntityPacket;
use crate::protocol_version::ProtocolVersion;
use std::io::Error;
use tokio::io::BufReader;
use tokio::net::tcp::OwnedReadHalf;

pub struct PickupSpawnPacket {
    pub entity: EntityPacket,
    pub item: i16,
    pub count: i8,
    pub item_damage: i16,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub rotation: i8,
    pub pitch: i8,
    pub roll: i8,
}

impl ServerPacket for PickupSpawnPacket {
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
            item: reader.read_i16(encryption).await?,
            count: reader.read_i8(encryption).await?,
            item_damage: reader.read_i16(encryption).await?,
            x: reader.read_i32(encryption).await?,
            y: reader.read_i32(encryption).await?,
            z: reader.read_i32(encryption).await?,
            rotation: reader.read_i8(encryption).await?,
            pitch: reader.read_i8(encryption).await?,
            roll: reader.read_i8(encryption).await?,
        })
    }
}
