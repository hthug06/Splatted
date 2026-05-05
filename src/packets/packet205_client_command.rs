use crate::packets::packet_trait::ClientPacket;
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
    fn write_to(&self, buffer: &mut Vec<u8>) -> Result<(), Error> {
        buffer.push(205);
        buffer.push(self.force_respawn);

        Ok(())
    }
}
