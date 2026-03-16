pub mod packet254_server_ping;
pub mod packet255_kick_disconnect;
pub mod packet2_client_protocol;

pub trait ServerPacket {
    fn read(buffer: &[u8]) -> Self;
}

pub trait ClientPacket {
    fn write(&self) -> Vec<u8>;
}
