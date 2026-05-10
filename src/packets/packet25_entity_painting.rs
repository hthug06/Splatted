use crate::network::connection::Encryption;
use crate::packets::packet_trait::ServerPacket;
use crate::packets::packet30_entity::EntityPacket;
use crate::packets::utils::{read_i32, read_string};
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
    ) -> Result<Self, Error>
    where
        Self: Sized,
    {
        Ok(Self {
            entity: EntityPacket::read(reader, encryption).await?,
            title: read_string(reader, encryption).await?,
            x: read_i32(reader, encryption).await?,
            y: read_i32(reader, encryption).await?,
            z: read_i32(reader, encryption).await?,
            direction: read_i32(reader, encryption).await?,
        })
    }
}
