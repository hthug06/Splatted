use crate::network::connection::Encryption;
use crate::packets::io::MinecraftReadExt;
use crate::packets::packet_trait::ServerPacket;
use std::io::Error;
use tokio::io::{AsyncReadExt, BufReader};
use tokio::net::tcp::OwnedReadHalf;

#[derive(Debug)]
pub struct CustomPayloadPacket {
    pub channel: String,
    pub length: i16,
    pub payload: Option<Vec<u8>>,
}

impl ServerPacket for CustomPayloadPacket {
    async fn read(
        reader: &mut BufReader<OwnedReadHalf>,
        encryption: &mut Encryption,
    ) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let channel = MinecraftReadExt::read_string(reader, encryption).await?;
        let length = MinecraftReadExt::read_i16(reader, encryption).await?;

        if length > 0 && length < 32767 {
            let mut payload = vec![0u8; length as usize];
            reader.read_exact(&mut payload).await?;
            encryption.decrypt(&mut payload);
            Ok(Self {
                channel,
                length,
                payload: Some(payload),
            })
        }
        // The size can be equal to 0
        else if length == 0 {
            Ok(Self {
                channel,
                length,
                payload: None,
            })
        } else {
            Err(Error::new(
                std::io::ErrorKind::InvalidData,
                format!(
                    "Payload size invalid or too large. Actual length: {}",
                    length
                ),
            ))
        }
    }
}
