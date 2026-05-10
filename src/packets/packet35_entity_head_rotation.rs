use crate::network::connection::Encryption;
use crate::packets::packet_trait::ServerPacket;
use crate::packets::packet30_entity::EntityPacket;
use std::io::Error;
use tokio::io::BufReader;
use tokio::net::tcp::OwnedReadHalf;

pub struct EntityHeadRotationPacket {
    pub entity_id: EntityPacket,
    pub head_rotation_yaw: i8,
}

impl ServerPacket for EntityHeadRotationPacket {
    async fn read(
        reader: &mut BufReader<OwnedReadHalf>,
        encryption: &mut Encryption,
    ) -> Result<Self, Error>
    where
        Self: Sized,
    {
        Ok(Self {
            // In the mc source code, this packet parse an i32 and set it to the entity_id
            // Because we created the EntityPacket Who parse an i32 and have an attribute named 'entity_id',
            // We might as well re-use it
            entity_id: EntityPacket::read(reader, encryption).await?,
            head_rotation_yaw: crate::packets::utils::read_i8(reader, encryption).await?,
        })
    }
}
