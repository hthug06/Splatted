use crate::network::connection::Encryption;
use crate::packets::packet_trait::ServerPacket;
use crate::packets::packet30_entity::EntityPacket;
use crate::packets::utils::read_i8;
use std::io::Error;
use tokio::io::BufReader;
use tokio::net::tcp::OwnedReadHalf;

pub struct EntityLookPacket {
    pub entity: EntityPacket,
    pub yaw: i8,
    pub pitch: i8,
}

impl ServerPacket for EntityLookPacket {
    async fn read(
        reader: &mut BufReader<OwnedReadHalf>,
        encryption: &mut Encryption,
    ) -> Result<Self, Error>
    where
        Self: Sized,
    {
        Ok(Self {
            entity: EntityPacket::read(reader, encryption).await?,
            yaw: read_i8(reader, encryption).await?,
            pitch: read_i8(reader, encryption).await?,
        })
    }
}
