use crate::network::connection::Encryption;
use crate::packets::io::MinecraftReadExt;
use crate::packets::packet_trait::ServerPacket;
use crate::packets::types::player_statistic::PlayerStatistic;
use crate::protocol_version::ProtocolVersion;
use std::io::Error;
use tokio::io::BufReader;
use tokio::net::tcp::OwnedReadHalf;

pub struct StatisticPacket {
    statistic: PlayerStatistic,
    value: i8,
}

impl ServerPacket for StatisticPacket {
    async fn read(
        reader: &mut BufReader<OwnedReadHalf>,
        encryption: &mut Encryption,
        _protocol_version: ProtocolVersion,
    ) -> Result<Self, Error>
    where
        Self: Sized,
    {
        Ok(Self {
            statistic: PlayerStatistic::from_id(reader.read_i32(encryption).await?),
            value: reader.read_i8(encryption).await?,
        })
    }
}
