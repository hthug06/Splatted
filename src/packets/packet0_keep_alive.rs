use crate::network::connection::Encryption;
use crate::packets::packet_trait::{ClientPacket, ServerPacket};
use crate::packets::utils::read_i32;
use std::io::Error;
use tokio::io::BufReader;
use tokio::net::tcp::OwnedReadHalf;

pub struct KeepAlive {
    pub random_id: i32,
}

impl ClientPacket for KeepAlive {
    fn write_to(&self, buffer: &mut Vec<u8>) -> Result<(), Error> {
        buffer.push(0);
        buffer.extend(self.random_id.to_be_bytes());
        Ok(())
    }
}

impl ServerPacket for KeepAlive {
    async fn read(
        reader: &mut BufReader<OwnedReadHalf>,
        encryption: &mut Encryption,
    ) -> Result<Self, Error>
    where
        Self: Sized,
    {
        Ok(Self {
            random_id: read_i32(reader, encryption).await?,
        })
    }
}
