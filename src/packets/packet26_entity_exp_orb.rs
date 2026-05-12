use crate::network::connection::Encryption;
use crate::packets::io::MinecraftReadExt;
use crate::packets::packet_trait::ServerPacket;
use crate::packets::packet30_entity::EntityPacket;
use crate::protocol_version::ProtocolVersion;
use std::io::Error;
use tokio::io::BufReader;
use tokio::net::tcp::OwnedReadHalf;

pub struct EntityExpOrbPacket {
    pub entity: EntityPacket,
    // We want to be precise on the coordinate of the entity orb, else they will be in the corner of the block
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub xp_value: i16,
}

impl ServerPacket for EntityExpOrbPacket {
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
            x: reader.read_i32(encryption).await? as f64 / 32.0,
            y: reader.read_i32(encryption).await? as f64 / 32.0,
            z: reader.read_i32(encryption).await? as f64 / 32.0,
            xp_value: reader.read_i16(encryption).await?,
        })
    }
}
