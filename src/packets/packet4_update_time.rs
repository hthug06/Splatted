use crate::network::connection::Encryption;
use crate::packets::io::MinecraftReadExt;
use crate::packets::packet_trait::ServerPacket;
use crate::protocol_version::ProtocolVersion;
use std::io::Error;
use tokio::io::BufReader;
use tokio::net::tcp::OwnedReadHalf;

pub struct UpdateTimePacket {
    /// Implemented in 1.4
    pub world_age: i64,
    pub time_of_day: i64,
}

impl ServerPacket for UpdateTimePacket {
    async fn read(
        reader: &mut BufReader<OwnedReadHalf>,
        encryption: &mut Encryption,
        protocol_version: ProtocolVersion,
    ) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let (world_age, time_of_day) = if protocol_version == ProtocolVersion::V1_4
            || protocol_version == ProtocolVersion::V1_5
            || protocol_version == ProtocolVersion::V1_6
        {
            let world_age = reader.read_i64(encryption).await?;
            let time_of_day = reader.read_i64(encryption).await?;
            (world_age, time_of_day)
        } else {
            // in 1.2 AND 1.3, there is no world age
            let time_of_day = reader.read_i64(encryption).await?;
            (0, time_of_day)
        };

        Ok(Self {
            world_age,
            time_of_day,
        })
    }
}
