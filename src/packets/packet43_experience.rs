use crate::network::connection::Encryption;
use crate::packets::io::MinecraftReadExt;
use crate::packets::packet_trait::ServerPacket;
use crate::protocol_version::ProtocolVersion;
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
        _protocol_version: ProtocolVersion,
    ) -> Result<Self, Error>
    where
        Self: Sized,
    {
        Ok(Self {
            experience: reader.read_f32(encryption).await?,
            experience_level: reader.read_i16(encryption).await?,
            experience_total: reader.read_i16(encryption).await?,
        })
    }
}
