use crate::packets::ClientPacket;
use std::io::Error;
#[derive(Default)]
pub struct ServerPing;

impl ClientPacket for ServerPing {
    fn write_to(&self, buffer: &mut Vec<u8>) -> Result<(), Error> {
        buffer.extend(vec![254, 1]);
        Ok(())
    }
}
