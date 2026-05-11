use crate::network::connection::Encryption;
use crate::packets::io::MinecraftReadExt;
use crate::packets::packet_trait::{ClientPacket, ServerPacket};
use crate::protocol_version::ProtocolVersion;
use bytes::{BufMut, BytesMut};
use std::io::Error;
use tokio::io::BufReader;
use tokio::net::tcp::OwnedReadHalf;

pub struct KeepAlivePacket {
    pub random_id: i32,
}

impl ClientPacket for KeepAlivePacket {
    fn write_to(
        &self,
        buffer: &mut BytesMut,
        _protocol_version: ProtocolVersion,
    ) -> Result<(), Error> {
        buffer.put_u8(0);
        buffer.put_i32(self.random_id);
        Ok(())
    }
}

impl ServerPacket for KeepAlivePacket {
    async fn read(
        reader: &mut BufReader<OwnedReadHalf>,
        encryption: &mut Encryption,
        _protocol_version: ProtocolVersion,
    ) -> Result<Self, Error>
    where
        Self: Sized,
    {
        Ok(Self {
            random_id: reader.read_i32(encryption).await?,
        })
    }
}
