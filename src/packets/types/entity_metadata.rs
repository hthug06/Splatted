use crate::network::connection::Encryption;
use crate::packets::io::MinecraftReadExt;
use crate::packets::types::itemstack::ItemStack;
use crate::protocol_version::ProtocolVersion;
use std::collections::HashMap;
use std::io::Error;
use tokio::io::BufReader;
use tokio::net::tcp::OwnedReadHalf;

/// All Possible metadata value
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
        protocol_version: ProtocolVersion,
    ) -> Result<Self, Error> {
        let mut entries = HashMap::new();

        loop {
            // The header byte is read (as unsigned for bitwise operations)
            let header = reader.read_u8(encryption).await?;

            // 127 (0x7F) say that it's the end of the Metadata
            if header == 127 {
                break;
            }

            // Bitwise operation for the data_type ° index
            let data_type = (header & 0xE0) >> 5;
            let index = header & 0x1F;

            // read_data for the type
            let value = match data_type {
                0 => MetadataValue::Byte(reader.read_i8(encryption).await?),
                1 => MetadataValue::Short(reader.read_i16(encryption).await?),
                2 => MetadataValue::Int(reader.read_i32(encryption).await?),
                3 => MetadataValue::Float(reader.read_f32(encryption).await?),
                4 => MetadataValue::String(reader.read_string(encryption).await?),
                5 => {
                    // From 1.4, they use itemstack.read
                    if protocol_version == ProtocolVersion::V1_4
                        || protocol_version == ProtocolVersion::V1_5
                        || protocol_version == ProtocolVersion::V1_6
                    {
                        MetadataValue::Item(ItemStack::read(reader, encryption).await?)
                    }
                    // Before, it's just id, stack_size and item damage
                    else {
                        MetadataValue::Item(ItemStack::new_simple(
                            reader.read_i16(encryption).await?,
                            Some(reader.read_u8(encryption).await?),
                            Some(reader.read_i16(encryption).await?),
                        ))
                    }
                }
                6 => MetadataValue::ChunkCoordinates(
                    reader.read_i32(encryption).await?,
                    reader.read_i32(encryption).await?,
                    reader.read_i32(encryption).await?,
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
