use crate::network::connection::Encryption;
use crate::packets::packet_trait::ServerPacket;
use crate::packets::types::player_statistic::PlayerStatistic;
use crate::packets::utils::{read_i8, read_i32};
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
    ) -> Result<Self, Error>
    where
        Self: Sized,
    {
        Ok(Self {
            statistic: PlayerStatistic::from_id(read_i32(reader, encryption).await?),
            value: read_i8(reader, encryption).await?,
        })
    }
}
