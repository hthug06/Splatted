use crate::network::connection::Encryption;
use crate::packets::packet_trait::ServerPacket;
use crate::packets::utils::{read_i16, read_string};
use std::io::Error;
use tokio::io::{AsyncReadExt, BufReader};
use tokio::net::tcp::OwnedReadHalf;

#[derive(Debug)]
pub struct CustomPayloadPacket {
    pub channel: String,
    pub length: i16,
    pub payload: Vec<u8>,
}

impl ServerPacket for CustomPayloadPacket {
    async fn read(
        reader: &mut BufReader<OwnedReadHalf>,
        encryption: &mut Encryption,
    ) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let channel = read_string(reader, encryption).await?;
        let length = read_i16(reader, encryption).await?;

        if length > 0 && length < 32767 {
            let mut payload = Vec::with_capacity(length as usize);
            reader.read_exact(&mut payload).await?;
            encryption.decrypt(&mut payload);
            Ok(Self {
                channel,
                length,
                payload,
            })
        } else {
            Err(Error::new(
                std::io::ErrorKind::InvalidData,
                format!(
                    "Payload may not be larger than 32k. Actual length: {}",
                    length
                ),
            ))
        }
    }
}
