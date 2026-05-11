use crate::packets::io::MinecraftWriteExt;
use crate::packets::packet_trait::ClientPacket;
use bytes::{BufMut, BytesMut};
use std::io::Error;

pub struct PlayerPositionPacket {
    pub x: f64,
    pub y: f64,
    pub stance: f64,
    pub z: f64,
    pub on_ground: bool, // Flatten the Flying Packet
}

impl ClientPacket for PlayerPositionPacket {
    fn write_to(&self, buffer: &mut BytesMut) -> Result<(), Error> {
        // Packet ID
        buffer.put_u8(11);

        // Packet Data
        buffer.put_f64(self.x);
        buffer.put_f64(self.y);
        buffer.put_f64(self.stance);
        buffer.put_f64(self.z);
        buffer.write_bool(self.on_ground);

        Ok(())
    }
}
