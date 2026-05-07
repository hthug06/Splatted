use crate::packets::packet_trait::ClientPacket;
use crate::packets::utils::{write_bool, write_f64};
use std::io::Error;

pub struct PlayerPositionPacket {
    pub x: f64,
    pub y: f64,
    pub stance: f64,
    pub z: f64,
    pub on_ground: bool, // Flatten the Flying Packet
}

impl ClientPacket for PlayerPositionPacket {
    fn write_to(&self, buffer: &mut Vec<u8>) -> Result<(), Error> {
        write_f64(buffer, self.x);
        write_f64(buffer, self.y);
        write_f64(buffer, self.stance);
        write_f64(buffer, self.z);
        write_bool(buffer, self.on_ground);

        Ok(())
    }
}
