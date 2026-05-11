use crate::packets::io::MinecraftWriteExt;
use crate::packets::packet_trait::ClientPacket;
use bytes::{BufMut, BytesMut};
use std::io::Error;

pub struct ClientProtocolPacket {
    /// This might change if we try to support more version ( > 1.10.2 is u16)
    pub protocol_version: u8,
    pub username: String,
    pub server_hostname: String,
    pub server_port: u32,
}

impl ClientProtocolPacket {
    pub fn new(
        protocol_version: u8,
        username: &str,
        server_hostname: String,
        server_port: u32,
    ) -> Self {
        Self {
            protocol_version,
            username: username.to_owned(),
            server_hostname,
            server_port,
        }
    }
}

impl ClientPacket for ClientProtocolPacket {
    fn write_to(&self, buffer: &mut BytesMut) -> Result<(), Error> {
        //DON'T FORGET TO ADD THE PACKET ID
        buffer.put_u8(0x02);

        // Add all the infos
        buffer.put_u8(self.protocol_version);
        buffer.write_string(&self.username)?;
        buffer.write_string(&self.server_hostname)?;
        buffer.extend(self.server_port.to_be_bytes());

        Ok(())
    }
}
