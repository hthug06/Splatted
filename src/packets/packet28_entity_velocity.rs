use crate::network::connection::Encryption;
use crate::packets::io::MinecraftReadExt;
use crate::packets::packet_trait::ServerPacket;
use crate::packets::packet30_entity::EntityPacket;
use crate::protocol_version::ProtocolVersion;
use std::io::Error;
use tokio::io::BufReader;
use tokio::net::tcp::OwnedReadHalf;

pub struct EntityVelocityPacket {
    pub entity: EntityPacket,
    pub velocity_x: i16,
    pub velocity_y: i16,
    pub velocity_z: i16,
}

impl ServerPacket for EntityVelocityPacket {
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
            velocity_x: reader.read_i16(encryption).await?,
            velocity_y: reader.read_i16(encryption).await?,
            velocity_z: reader.read_i16(encryption).await?,
        })
    }
}
