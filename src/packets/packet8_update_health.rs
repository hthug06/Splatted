use crate::network::connection::Encryption;
use crate::packets::io::MinecraftReadExt;
use crate::packets::packet_trait::ServerPacket;
use crate::protocol_version::ProtocolVersion;
use std::io::Error;
use tokio::io::BufReader;
use tokio::net::tcp::OwnedReadHalf;

pub struct UpdateHealthPacket {
    /// < 1.6 : i16
    /// >= 1.6 : f32
    pub health: f32,
    pub food: i16,
    pub food_saturation: f32,
}

impl ServerPacket for UpdateHealthPacket {
    async fn read(
        reader: &mut BufReader<OwnedReadHalf>,
        encryption: &mut Encryption,
        protocol_version: ProtocolVersion,
    ) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let health = if protocol_version == ProtocolVersion::V1_6 {
            reader.read_f32(encryption).await?
        } else {
            reader.read_i16(encryption).await? as f32
        };
        let food = reader.read_i16(encryption).await?;
        let food_saturation = reader.read_f32(encryption).await?;

        Ok(Self {
            health,
            food,
            food_saturation,
        })
    }
}
