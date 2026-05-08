use crate::network::connection::Encryption;
use crate::packets::packet_trait::ServerPacket;
use crate::packets::utils::{read_f32, read_i16};
use std::io::Error;
use tokio::io::BufReader;
use tokio::net::tcp::OwnedReadHalf;

#[derive(Debug)]
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
            health: read_i16(reader, encryption).await?,
            food: read_i16(reader, encryption).await?,
            food_saturation: read_f32(reader, encryption).await?,
        })
    }
}
