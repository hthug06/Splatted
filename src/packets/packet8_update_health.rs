use crate::network::connection::Encryption;
use crate::packets::io::MinecraftReadExt;
use crate::packets::packet_trait::ServerPacket;
use std::io::Error;
use tokio::io::BufReader;
use tokio::net::tcp::OwnedReadHalf;

pub struct UpdateHealthPacket {
    pub health: i16,
    pub food: i16,
    pub food_saturation: f32,
}

impl ServerPacket for UpdateHealthPacket {
    async fn read(
        reader: &mut BufReader<OwnedReadHalf>,
        encryption: &mut Encryption,
    ) -> Result<Self, Error>
    where
        Self: Sized,
    {
        Ok(Self {
            health: reader.read_i16(encryption).await?,
            food: reader.read_i16(encryption).await?,
            food_saturation: reader.read_f32(encryption).await?,
        })
    }
}
