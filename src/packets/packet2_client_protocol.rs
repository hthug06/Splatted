use crate::packets::ClientPacket;
use crate::packets::write_string;
use std::io::Error;

pub struct ClientProtocol {
    /// This might change if we try to support more version ( > 1.10.2 is u16)
    pub protocol_version: u8,
    pub username: String,
    pub server_hostname: String,
    pub server_port: u32,
}

impl ClientProtocol {
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

    fn create_payload(&self) -> Result<Vec<u8>, Error> {
        let mut buffer: Vec<u8> = vec![];
        buffer.push(self.protocol_version);
        write_string(&mut buffer, &self.username)?;
        write_string(&mut buffer, &self.server_hostname)?;
        buffer.extend(self.server_port.to_be_bytes());
        Ok(buffer)
    }
}

impl ClientPacket for ClientProtocol {
    fn write_to(&self, buffer: &mut Vec<u8>) -> Result<(), Error> {
        //DON'T FORGET TO ADD THE PACKET ID
        buffer.push(0x02);

        // Add all the infos
        buffer.extend(self.create_payload()?);

        Ok(())
    }
}
