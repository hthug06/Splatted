use crate::packets::packet_trait::ClientPacket;
use std::io::Error;

pub struct ServerPingPacket;

impl ClientPacket for ServerPingPacket {
    fn write_to(&self, buffer: &mut Vec<u8>) -> Result<(), Error> {
        buffer.extend(vec![254, 1]);
        Ok(())
    }
}
