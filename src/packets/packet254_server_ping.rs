use crate::packets::ClientPacket;
#[derive(Default)]
pub struct ServerPing;

impl ClientPacket for ServerPing {
    fn write(&self) -> Vec<u8> {
        vec![254, 1]
    }
}
