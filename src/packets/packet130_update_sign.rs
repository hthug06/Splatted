use crate::network::connection::Encryption;
use crate::packets::packet_trait::ServerPacket;
use crate::packets::utils::{read_i16, read_i32, read_string};
use std::io::Error;
use tokio::io::BufReader;
use tokio::net::tcp::OwnedReadHalf;

#[derive(Debug)]
pub struct UpdateSignPacket {
    pub x: i32,
    pub y: i16,
    pub z: i32,
    // If the line is empty, use None
    pub sign_line: Vec<Option<String>>,
}

impl ServerPacket for UpdateSignPacket {
    async fn read(
        reader: &mut BufReader<OwnedReadHalf>,
        encryption: &mut Encryption,
    ) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let x = read_i32(reader, encryption).await?;
        let y = read_i16(reader, encryption).await?;
        let z = read_i32(reader, encryption).await?;

        // Read all the line of the sign (only 4)
        let mut sign_line = Vec::with_capacity(4);
        for _ in 0..4 {
            let read_line = read_string(reader, encryption).await?;
            let line = if read_line.is_empty() {
                None
            } else {
                Some(read_line)
            };

            sign_line.push(line);
        }

        Ok(Self { x, y, z, sign_line })
    }
}
