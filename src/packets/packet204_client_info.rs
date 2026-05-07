use crate::packets::packet_trait::ClientPacket;
use crate::packets::utils::{write_bool, write_string};
use std::io::Error;

// Petit rappel des constantes utiles pour ce paquet
pub const VIEW_DISTANCE_FAR: u8 = 0;
pub const VIEW_DISTANCE_NORMAL: u8 = 1;
pub const VIEW_DISTANCE_SHORT: u8 = 2;
pub const VIEW_DISTANCE_TINY: u8 = 3;

pub struct ClientInfoPacket {
    /// ex: "fr_FR" or "en_US"
    pub locale: String,
    /// 0 = Far, 1 = Normal, 2 = Short, 3 = Tiny (use const)
    pub view_distance: u8,
    /// 0 = Enabled, 1 = Commands Only, 2 = Hidden
    pub chat_flags: u8,
    /// true = Colors activated
    pub chat_colors: bool,
    /// 0 = Peaceful, 1 = Easy, 2 = Normal, 3 = Hard
    pub difficulty: u8,
    pub show_cape: bool,
}

impl ClientPacket for ClientInfoPacket {
    fn write_to(&self, buffer: &mut Vec<u8>) -> Result<(), Error> {
        write_string(buffer, &self.locale)?;
        buffer.push(self.view_distance);

        // Use bitmask for chat flags and colors
        let mut chat_byte = self.chat_flags & 3;
        if self.chat_colors {
            chat_byte |= 8;
        }
        buffer.push(chat_byte);

        buffer.push(self.difficulty);
        write_bool(buffer, self.show_cape);
        Ok(())
    }
}
