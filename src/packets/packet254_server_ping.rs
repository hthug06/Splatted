use crate::packets::packet_trait::ClientPacket;
use bytes::{BufMut, BytesMut};
use std::io::Error;

pub struct ServerPingPacket;

impl ClientPacket for ServerPingPacket {
    fn write_to(&self, buffer: &mut BytesMut) -> Result<(), Error> {
        buffer.put_u8(254);
        buffer.put_u8(1);
        Ok(())
    }
}
