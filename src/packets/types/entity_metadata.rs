use crate::network::connection::Encryption;
use crate::packets::types::itemstack::ItemStack;
use std::collections::HashMap;
use std::io::Error;
use tokio::io::BufReader;
use tokio::net::tcp::OwnedReadHalf;
// N'oublie pas d'importer tes utilitaires et ton ItemStack
use crate::packets::utils::{read_f32, read_i8, read_i16, read_i32, read_string, read_u8};

/// All Possible metadata value
#[derive(Debug)]
pub enum MetadataValue {
    Byte(i8),
    Short(i16),
    Int(i32),
    Float(f32),
    String(String),
    Item(Option<ItemStack>), // Option because for exemple: a zombie can have a sword, or not
    ChunkCoordinates(i32, i32, i32),
}

/// HashMap with all the entityMetaData
/// It's like the DataWatcher in mc code
#[derive(Debug)]
pub struct EntityMetadata {
    /// Key: Index
    /// Value: Data Type
    pub entries: HashMap<u8, MetadataValue>,
}

impl EntityMetadata {
    /// Read all the entity Metadata to create the object'
    pub async fn read(
        reader: &mut BufReader<OwnedReadHalf>,
        encryption: &mut Encryption,
    ) -> Result<Self, Error> {
        let mut entries = HashMap::new();

        loop {
            // The header byte is read (as unsigned for bitwise operations)
            let header = read_u8(reader, encryption).await?;

            // 127 (0x7F) say that it's the end of the Metadata
            if header == 127 {
                break;
            }

            // Bitwise operation for the data_type ° index
            let data_type = (header & 0xE0) >> 5;
            let index = header & 0x1F;

            // read_data for the type
            let value = match data_type {
                0 => MetadataValue::Byte(read_i8(reader, encryption).await?),
                1 => MetadataValue::Short(read_i16(reader, encryption).await?),
                2 => MetadataValue::Int(read_i32(reader, encryption).await?),
                3 => MetadataValue::Float(read_f32(reader, encryption).await?),
                4 => MetadataValue::String(read_string(reader, encryption).await?),
                5 => MetadataValue::Item(ItemStack::read(reader, encryption).await?),
                6 => MetadataValue::ChunkCoordinates(
                    read_i32(reader, encryption).await?,
                    read_i32(reader, encryption).await?,
                    read_i32(reader, encryption).await?,
                ),
                _ => {
                    return Err(Error::new(
                        std::io::ErrorKind::InvalidData,
                        format!("Unknown metadata type: {}", data_type),
                    ));
                }
            };

            entries.insert(index, value);
        }

        Ok(Self { entries })
    }
}
