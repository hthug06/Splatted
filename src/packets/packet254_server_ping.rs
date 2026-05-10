use crate::packets::packet_trait::ClientPacket;
use bytes::BytesMut;
use std::io::Error;

pub struct ServerPingPacket;

impl ClientPacket for ServerPingPacket {
    fn write_to(&self, buffer: &mut BytesMut) -> Result<(), Error> {
        buffer.extend(vec![254, 1]);
        Ok(())
    }
}
