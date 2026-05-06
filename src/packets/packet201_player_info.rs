use crate::network::connection::Encryption;
use crate::packets::packet_trait::ServerPacket;
use crate::packets::utils::{read_i16, read_string, read_u8};
use std::io::Error;
use tokio::io::BufReader;
use tokio::net::tcp::OwnedReadHalf;

#[derive(Debug)]
pub struct PlayerInfo {
    pub name: String,
    pub is_connected: bool,
    pub ping: i16,
}

impl ServerPacket for PlayerInfo {
    async fn read(
        reader: &mut BufReader<OwnedReadHalf>,
        encryption: &mut Encryption,
    ) -> Result<Self, Error>
    where
        Self: Sized,
    {
        Ok(Self {
            name: read_string(reader, encryption).await?,
            is_connected: read_u8(reader, encryption).await? != 0,
            ping: read_i16(reader, encryption).await?,
        })
    }
}
