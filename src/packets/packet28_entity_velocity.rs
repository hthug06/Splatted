use crate::network::connection::Encryption;
use crate::packets::packet_trait::ServerPacket;
use crate::packets::packet30_entity::EntityPacket;
use crate::packets::utils::read_i16;
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
    ) -> Result<Self, Error>
    where
        Self: Sized,
    {
        Ok(Self {
            entity: EntityPacket::read(reader, encryption).await?,
            velocity_x: read_i16(reader, encryption).await?,
            velocity_y: read_i16(reader, encryption).await?,
            velocity_z: read_i16(reader, encryption).await?,
        })
    }
}
