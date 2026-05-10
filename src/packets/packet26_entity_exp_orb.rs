use crate::network::connection::Encryption;
use crate::packets::packet_trait::ServerPacket;
use crate::packets::packet30_entity::EntityPacket;
use crate::packets::utils::{read_i16, read_i32};
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
    ) -> Result<Self, Error>
    where
        Self: Sized,
    {
        Ok(Self {
            entity: EntityPacket::read(reader, encryption).await?,
            x: read_i32(reader, encryption).await? as f64 / 32.0,
            y: read_i32(reader, encryption).await? as f64 / 32.0,
            z: read_i32(reader, encryption).await? as f64 / 32.0,
            xp_value: read_i16(reader, encryption).await?,
        })
    }
}
