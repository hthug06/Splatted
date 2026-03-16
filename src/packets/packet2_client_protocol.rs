use crate::packets::ClientPacket;
use crate::utils::write_string;

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
        username: &String,
        server_hostname: String,
        server_port: u32,
    ) -> Self {
        Self {
            protocol_version,
            username: username.clone(),
            server_hostname,
            server_port,
        }
    }

    fn create_payload(&self) -> Vec<u8> {
        let mut buffer: Vec<u8> = vec![];
        buffer.push(self.protocol_version);
        write_string(&mut buffer, &self.username).unwrap();
        write_string(&mut buffer, &self.server_hostname).unwrap();
        buffer.extend(self.server_port.to_be_bytes());
        buffer
    }
}

impl ClientPacket for ClientProtocol {
    fn write(&self) -> Vec<u8> {
        //Create the buffer
        let mut buffer: Vec<u8> = Vec::new();

        //DON'T FORGET TO ADD THE PACKET ID
        buffer.push(0x02);

        // Add all the infos
        buffer.extend_from_slice(self.create_payload().as_slice());
        buffer
    }
}
