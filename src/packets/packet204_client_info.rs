use crate::packets::packet_trait::ClientPacket;
use crate::packets::utils::{write_bool, write_string};
use std::io::Error;

pub struct ClientInfoPacket {
    /// ex: "fr_FR" or "en_US"
    pub locale: String,
    /// 0 = Far, 1 = Normal, 2 = Short, 3 = Tiny
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
        // Packet ID
        buffer.push(204);

        // Packet Data
        write_string(buffer, &self.locale)?;
        buffer.push(self.view_distance);

        // Using real Minecraft implementation (or MCP I don't really know ?)
        // Java code:
        // par1DataOutputStream.writeByte(this.chatVisisble | (this.chatColours ? 1 : 0) << 3);
        buffer.push(self.chat_flags | (if self.chat_colors { 1 } else { 0 }) << 3);

        buffer.push(self.difficulty);
        write_bool(buffer, self.show_cape);
        Ok(())
    }
}
