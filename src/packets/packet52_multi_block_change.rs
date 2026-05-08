use crate::network::connection::Encryption;
use crate::packets::packet_trait::ServerPacket;
use crate::packets::utils::{read_i32, read_u16};
use std::io::{Error, ErrorKind};
use tokio::io::{AsyncReadExt, BufReader};
use tokio::net::tcp::OwnedReadHalf;

const MAX_METADATA_SIZE: i32 = 32767;

#[derive(Debug)]
pub struct MultiBlockChangePacket {
    pub x: i32,
    pub z: i32,
    pub size: u16,
    pub metadata: Option<Vec<u8>>,
}

impl ServerPacket for MultiBlockChangePacket {
    async fn read(
        reader: &mut BufReader<OwnedReadHalf>,
        encryption: &mut Encryption,
    ) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let x = read_i32(reader, encryption).await?;
        let z = read_i32(reader, encryption).await?;

        // In the mc source code, they use & 65535
        // Because we have unsigned int, we don't need to do it
        let size = read_u16(reader, encryption).await?;

        // the metadata is all the block who changed
        // The size of the metadata is precised, so we need to read it first
        let metadata_size = read_i32(reader, encryption).await?;

        // After that, if the size > 0, there is metadata
        let metadata = if metadata_size > 0 {
            // Because we have the size, we can read it
            let mut metadata = vec![0; metadata_size as usize];
            reader.read_exact(&mut metadata).await?;

            // Don't forget to decrypt
            encryption.decrypt(&mut metadata);

            Some(metadata)
        }
        // If the server is malicious and what to make su crash
        else if metadata_size > MAX_METADATA_SIZE {
            return Err(Error::new(
                ErrorKind::InvalidData,
                format!("Metadata size is too big: {}", metadata_size),
            ));
        } else {
            None
        };

        Ok(Self {
            x,
            z,
            size,
            metadata,
        })
    }
}
