use crate::packets::ClientPacket;

pub struct ServerPing;

impl ClientPacket for ServerPing {
    fn write() -> Vec<u8> {
        vec![254, 1]
    }
}
