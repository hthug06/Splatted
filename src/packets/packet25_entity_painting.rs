use crate::network::connection::Encryption;
use crate::packets::io::MinecraftReadExt;
use crate::packets::packet_trait::ServerPacket;
use crate::packets::packet30_entity::EntityPacket;
use crate::protocol_version::ProtocolVersion;
use std::io::Error;
use tokio::io::BufReader;
use tokio::net::tcp::OwnedReadHalf;

pub struct EntityPaintingPacket {
    pub entity: EntityPacket,
    pub title: String,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub direction: i32,
}

impl ServerPacket for EntityPaintingPacket {
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
            title: reader.read_string(encryption).await?, // TODO: create an enum like EnumArt
            x: reader.read_i32(encryption).await?,
            y: reader.read_i32(encryption).await?,
            z: reader.read_i32(encryption).await?,
            direction: reader.read_i32(encryption).await?,
        })
    }
}
