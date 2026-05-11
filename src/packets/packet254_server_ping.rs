use crate::packets::packet_trait::ClientPacket;
use crate::protocol_version::ProtocolVersion;
use bytes::{BufMut, BytesMut};
use std::io::Error;

pub struct ServerPingPacket;

impl ClientPacket for ServerPingPacket {
    fn write_to(
        &self,
        buffer: &mut BytesMut,
        _protocol_version: ProtocolVersion,
    ) -> Result<(), Error> {
        buffer.put_u8(254);
        buffer.put_u8(1);
        Ok(())
    }
}
