use crate::network::connection::Encryption;
use crate::packets::packet_trait::ServerPacket;
use crate::packets::packet30_entity::EntityPacket;
use crate::packets::utils::read_i8;
use std::io::Error;
use tokio::io::BufReader;
use tokio::net::tcp::OwnedReadHalf;

pub struct RelEntityMoveLookPacket {
    pub entity: EntityPacket,
    pub x: i8,
    pub y: i8,
    pub z: i8,
    pub yaw: i8,
    pub pitch: i8,
}

impl ServerPacket for RelEntityMoveLookPacket {
    async fn read(
        reader: &mut BufReader<OwnedReadHalf>,
        encryption: &mut Encryption,
    ) -> Result<Self, Error>
    where
        Self: Sized,
    {
        Ok(Self {
            entity: EntityPacket::read(reader, encryption).await?,
            x: read_i8(reader, encryption).await?,
            y: read_i8(reader, encryption).await?,
            z: read_i8(reader, encryption).await?,
            yaw: read_i8(reader, encryption).await?,
            pitch: read_i8(reader, encryption).await?,
        })
    }
}
