use crate::network::connection::Encryption;
use crate::packets::io::MinecraftReadExt;
use crate::packets::packet_trait::ServerPacket;
use crate::protocol_version::ProtocolVersion;
use bytes::{Bytes, BytesMut};
use std::io::{Error, ErrorKind};
use tokio::io::{AsyncReadExt, BufReader};
use tokio::net::tcp::OwnedReadHalf;

/// Represent a chunk
pub struct MapChunkPacket {
    pub x: i32,
    pub z: i32,
    pub continuous: bool,
    pub primary_bitmap: u16,
    pub add_bitmap: u16,
    pub compressed_data: Bytes,
}

impl ServerPacket for MapChunkPacket {
    async fn read(
        reader: &mut BufReader<OwnedReadHalf>,
        encryption: &mut Encryption,
        protocol_version: ProtocolVersion,
    ) -> Result<Self, Error>
    where
        Self: Sized,
    {
        // For more documentation, go see the packet 56

        let x = MinecraftReadExt::read_i32(reader, encryption).await?;
        let z = MinecraftReadExt::read_i32(reader, encryption).await?;

        // True = the whole chunk
        // False = only a certain part of the chunk (ex: when a big explosion happen)
        let continuous = MinecraftReadExt::read_i8(reader, encryption).await? != 0;

        let primary_bitmap = MinecraftReadExt::read_i16(reader, encryption).await? as u16;
        let add_bitmap = MinecraftReadExt::read_i16(reader, encryption).await? as u16;

        // The size of the compressed data
        let compressed_size = MinecraftReadExt::read_i32(reader, encryption).await?;

        if protocol_version == ProtocolVersion::V1_2 {
            // Useless but we need to read it to avoid desync
            let _decompressed_size = MinecraftReadExt::read_i32(reader, encryption).await?;
        }

        // Security
        if compressed_size < 0 || compressed_size > 2_000_000 {
            return Err(Error::new(
                ErrorKind::InvalidData,
                format!("Invalid chunk size: {}", compressed_size),
            ));
        }

        // ZERO-COPY
        // Create a buffer with the right size
        let mut compressed_buffer = BytesMut::zeroed(compressed_size as usize);

        if compressed_size > 0 {
            reader.read_exact(&mut compressed_buffer).await?;
            encryption.decrypt(&mut compressed_buffer);
        }

        // Freeze the memory so it's not deleted
        let compressed_data = compressed_buffer.freeze();

        Ok(Self {
            x,
            z,
            continuous,
            primary_bitmap,
            add_bitmap,
            compressed_data,
        })
    }
}
