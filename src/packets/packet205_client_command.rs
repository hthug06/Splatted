use crate::packets::packet_trait::ClientPacket;
use bytes::{BufMut, BytesMut};
use std::io::Error;

pub struct ClientCommandPacket {
    pub force_respawn: u8,
}

impl ClientCommandPacket {
    pub fn new(force_respawn: u8) -> Self {
        Self { force_respawn }
    }
}

impl ClientPacket for ClientCommandPacket {
    fn write_to(&self, buffer: &mut BytesMut) -> Result<(), Error> {
        buffer.put_u8(205);
        buffer.put_u8(self.force_respawn);

        Ok(())
    }
}
