use crate::packets::packet_trait::ClientPacket;
use crate::packets::utils::{write_bool, write_f32};
use std::io::Error;

pub struct PlayerLookPacket {
    pub yaw: f32,
    pub pitch: f32,
    pub on_ground: bool, // Flatten the Flying packet
}

impl ClientPacket for PlayerLookPacket {
    fn write_to(&self, buffer: &mut Vec<u8>) -> Result<(), Error> {
        // Packet ID
        buffer.push(12);

        // Packet data
        write_f32(buffer, self.yaw);
        write_f32(buffer, self.pitch);
        write_bool(buffer, self.on_ground);
        Ok(())
    }
}
