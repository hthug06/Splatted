use crate::network::connection::Encryption;
use crate::packets::packet_trait::ServerPacket;
use crate::packets::utils::{read_i16, read_i32};
use std::io::Error;
use tokio::io::BufReader;
use tokio::net::tcp::OwnedReadHalf;

#[derive(Debug)]
pub struct BlockItemSwitch {
    id: i32,
}

impl ServerPacket for BlockItemSwitch {
    async fn read(
        reader: &mut BufReader<OwnedReadHalf>,
        encryption: &mut Encryption,
    ) -> Result<Self, Error>
    where
        Self: Sized,
    {
        Ok(Self {
            id: read_i16(reader, encryption).await? as i32,
        })
    }
}
