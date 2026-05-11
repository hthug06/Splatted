use crate::packets::io::MinecraftWriteExt;
use crate::packets::packet_trait::ClientPacket;
use bytes::{BufMut, BytesMut};
use std::io::Error;

pub struct PlayerLookPacket {
    pub yaw: f32,
    pub pitch: f32,
    pub on_ground: bool, // Flatten the Flying packet
}

impl ClientPacket for PlayerLookPacket {
    fn write_to(&self, buffer: &mut BytesMut) -> Result<(), Error> {
        // Packet ID
        buffer.put_u8(12);

        // Packet data
        buffer.put_f32(self.yaw);
        buffer.put_f32(self.pitch);
        buffer.write_bool(self.on_ground);
        Ok(())
    }
}
