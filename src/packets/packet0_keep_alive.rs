use crate::network::connection::Encryption;
use crate::packets::packet_trait::{ClientPacket, ServerPacket};
use crate::packets::utils::read_i32;
use bytes::{BufMut, BytesMut};
use std::io::Error;
use tokio::io::BufReader;
use tokio::net::tcp::OwnedReadHalf;

pub struct KeepAlivePacket {
    pub random_id: i32,
}

impl ClientPacket for KeepAlivePacket {
    fn write_to(&self, buffer: &mut BytesMut) -> Result<(), Error> {
        buffer.put_u8(0);
        buffer.extend(self.random_id.to_be_bytes());
        Ok(())
    }
}

impl ServerPacket for KeepAlivePacket {
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
