use crate::network::connection::Encryption;
use crate::packets::packet_trait::ServerPacket;
use crate::packets::utils::read_i64;
use std::io::Error;
use tokio::io::BufReader;
use tokio::net::tcp::OwnedReadHalf;

pub struct UpdateTimePacket {
    pub world_age: i64,
    pub time_of_day: i64,
}

impl ServerPacket for UpdateTimePacket {
    async fn read(
        reader: &mut BufReader<OwnedReadHalf>,
        encryption: &mut Encryption,
    ) -> Result<Self, Error>
    where
        Self: Sized,
    {
        Ok(Self {
            world_age: read_i64(reader, encryption).await?,
            time_of_day: read_i64(reader, encryption).await?,
        })
    }
}
