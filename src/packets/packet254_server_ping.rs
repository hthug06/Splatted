use crate::packets::ClientPacket;
use std::io::Error;
pub struct ServerPing;

impl ClientPacket for ServerPing {
    fn write_to(&self, buffer: &mut Vec<u8>) -> Result<(), Error> {
        buffer.extend(vec![254, 1]);
        Ok(())
    }
}
