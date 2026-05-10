use crate::network::connection::Encryption;
use crate::packets::packet_trait::ServerPacket;
use crate::packets::packet30_entity::EntityPacket;
use crate::packets::utils::{read_i32, read_u8};
use std::io::Error;
use tokio::io::BufReader;
use tokio::net::tcp::OwnedReadHalf;

pub struct WeatherPacket {
    pub entity: EntityPacket,
    pub is_lightning_bolt: bool,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl ServerPacket for WeatherPacket {
    async fn read(
        reader: &mut BufReader<OwnedReadHalf>,
        encryption: &mut Encryption,
    ) -> Result<Self, Error>
    where
        Self: Sized,
    {
        Ok(Self {
            entity: EntityPacket::read(reader, encryption).await?,
            is_lightning_bolt: read_u8(reader, encryption).await? == 1,
            x: read_i32(reader, encryption).await? as f64 / 32.0,
            y: read_i32(reader, encryption).await? as f64 / 32.0,
            z: read_i32(reader, encryption).await? as f64 / 32.0,
        })
    }
}
