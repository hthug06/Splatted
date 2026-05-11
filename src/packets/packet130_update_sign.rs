use crate::network::connection::Encryption;
use crate::packets::io::MinecraftReadExt;
use crate::packets::packet_trait::ServerPacket;
use std::io::Error;
use tokio::io::BufReader;
use tokio::net::tcp::OwnedReadHalf;

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
        let x = reader.read_i32(encryption).await?;
        let y = reader.read_i16(encryption).await?;
        let z = reader.read_i32(encryption).await?;

        // Read all the line of the sign (only 4)
        let mut sign_line = Vec::with_capacity(4);
        for _ in 0..4 {
            let read_line = reader.read_string(encryption).await?;
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
