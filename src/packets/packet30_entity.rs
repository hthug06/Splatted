use crate::network::connection::Encryption;
use crate::packets::packet_trait::ServerPacket;
use crate::packets::utils::read_i32;
use std::io::Error;
use tokio::io::BufReader;
use tokio::net::tcp::OwnedReadHalf;
#[derive(Debug)]
pub struct EntityPacket {
    pub entity_id: i32,
}

impl ServerPacket for EntityPacket {
    async fn read(
        reader: &mut BufReader<OwnedReadHalf>,
        encryption: &mut Encryption,
    ) -> Result<Self, Error>
    where
        Self: Sized,
    {
        Ok(Self {
            entity_id: read_i32(reader, encryption).await?,
        })
    }
}
