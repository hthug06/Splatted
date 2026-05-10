use crate::network::connection::Encryption;
use crate::packets::packet_trait::ServerPacket;
use crate::packets::utils::{read_f32, read_i16};
use std::io::Error;
use tokio::io::BufReader;
use tokio::net::tcp::OwnedReadHalf;

pub struct ExperiencePacket {
    pub experience: f32,
    pub experience_level: i16,
    pub experience_total: i16,
}

impl ServerPacket for ExperiencePacket {
    async fn read(
        reader: &mut BufReader<OwnedReadHalf>,
        encryption: &mut Encryption,
    ) -> Result<Self, Error>
    where
        Self: Sized,
    {
        Ok(Self {
            experience: read_f32(reader, encryption).await?,
            experience_level: read_i16(reader, encryption).await?,
            experience_total: read_i16(reader, encryption).await?,
        })
    }
}
