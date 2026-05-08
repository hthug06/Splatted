//! The Map Chunk Packet
//! For now, we parse everything, but we do nothing with the data
use crate::network::connection::Encryption;
use crate::packets::packet_trait::ServerPacket;
use crate::packets::utils::{read_i8, read_i16, read_i32, read_u16};
use std::io::{Error, ErrorKind};
use tokio::io::{AsyncReadExt, BufReader};
use tokio::net::tcp::OwnedReadHalf;

// Because 1 chunk can be max 1 Mo, here considering modded server , 20 BIG chunk should be enough (20 Mo max)
const MAX_METADATA_SIZE: i32 = 20_971_520;

/// The packet MapChunk
/// This packet contain:
/// - The number of chunk sent
/// - The data lenght of the combined chunk
/// - If the sky light is sent (ex: sent in overworld, not in the nether)
#[derive(Debug)]
pub struct MapChunkPacket {
    pub chunk_count: i16,
    pub data_length: i32,
    pub sky_light_sent: bool,
    pub compressed_data: Vec<u8>,
    pub metadata: Vec<ChunkMetaData>,
}

#[derive(Debug)]
pub struct ChunkMetaData {
    pub chunk_x: i32,
    pub chunk_z: i32,
    pub primary_bitmap: u16,
    pub add_bitmap: u16,
}

impl ServerPacket for MapChunkPacket {
    async fn read(
        reader: &mut BufReader<OwnedReadHalf>,
        encryption: &mut Encryption,
    ) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let chunk_count = read_i16(reader, encryption).await?;
        let data_length = read_i32(reader, encryption).await?;
        if data_length > MAX_METADATA_SIZE {
            return Err(Error::new(
                ErrorKind::InvalidData,
                format!("Bulk Chunk size is too big: {}", data_length),
            ));
        }

        let sky_light_sent = read_i8(reader, encryption).await? != 0;

        // Read compressed data
        // Data is compressed in ZLib
        // If we want to decompress this, we need to use a crate like flate2
        //
        // The compressed data contain the chunk data of X chunk.
        // To read it (need to be decompressed), it's :
        // - chunk 1: read Y number of byte
        // - chunk 2: read the next Y number of byte
        // - ...
        // To know where we need to cut, we get the bitmap (check after)
        //
        // This compressed data is where all the 'chunk data' is:
        // block, block metadata, light (from block and sky)
        // + everything are in vec (vec of block, vec of block metadata...)
        let mut compressed_data = vec![0u8; data_length as usize];
        if data_length > 0 {
            reader.read_exact(&mut compressed_data).await?;
            encryption.decrypt(&mut compressed_data);
        }

        // Read EVERY chunk (yes multiple chunk in 1 packet)
        let mut metadata = Vec::new();
        for _ in 0..chunk_count {
            metadata.push(ChunkMetaData {
                chunk_x: read_i32(reader, encryption).await?,
                chunk_z: read_i32(reader, encryption).await?,
                primary_bitmap: read_u16(reader, encryption).await?,
                add_bitmap: read_u16(reader, encryption).await?,
            });
        }

        Ok(Self {
            chunk_count,
            data_length,
            sky_light_sent,
            compressed_data,
            metadata,
        })
    }
}
