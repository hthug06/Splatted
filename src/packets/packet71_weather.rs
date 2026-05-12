use crate::network::connection::Encryption;
use crate::packets::io::MinecraftReadExt;
use crate::packets::packet_trait::ServerPacket;
use crate::packets::packet30_entity::EntityPacket;
use crate::protocol_version::ProtocolVersion;
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
        protocol_version: ProtocolVersion,
    ) -> Result<Self, Error>
    where
        Self: Sized,
    {
        Ok(Self {
            entity: EntityPacket::read(reader, encryption, protocol_version).await?,
            is_lightning_bolt: reader.read_u8(encryption).await? == 1,
            x: reader.read_i32(encryption).await? as f64 / 32.0,
            y: reader.read_i32(encryption).await? as f64 / 32.0,
            z: reader.read_i32(encryption).await? as f64 / 32.0,
        })
    }
}
