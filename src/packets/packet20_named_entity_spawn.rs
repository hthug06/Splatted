use crate::network::connection::Encryption;
use crate::packets::io::MinecraftReadExt;
use crate::packets::packet_trait::ServerPacket;
use crate::packets::packet30_entity::EntityPacket;
use crate::packets::types::entity_metadata::EntityMetadata;
use crate::packets::types::itemstack::ItemStack;
use crate::protocol_version::ProtocolVersion;
use std::io::Error;
use tokio::io::BufReader;
use tokio::net::tcp::OwnedReadHalf;

pub struct NamedEntitySpawnPacket {
    pub entity: EntityPacket,
    pub name: String,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub rotation: i8,
    pub pitch: i8,
    pub current_item: Option<ItemStack>,
    /// Implemented in 1.3
    pub metadata: Option<EntityMetadata>,
}

impl ServerPacket for NamedEntitySpawnPacket {
    async fn read(
        reader: &mut BufReader<OwnedReadHalf>,
        encryption: &mut Encryption,
        protocol_version: ProtocolVersion,
    ) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let entity = EntityPacket::read(reader, encryption, protocol_version).await?;
        let name = reader.read_string(encryption).await?;
        let x = reader.read_i32(encryption).await?;
        let y = reader.read_i32(encryption).await?;
        let z = reader.read_i32(encryption).await?;
        let rotation = reader.read_i8(encryption).await?;
        let pitch = reader.read_i8(encryption).await?;
        let current_item = ItemStack::new_simple(reader.read_i16(encryption).await?, None, None);

        let metadata = if protocol_version == ProtocolVersion::V1_3
            || protocol_version == ProtocolVersion::V1_4
        {
            Some(EntityMetadata::read(reader, encryption, protocol_version).await?)
        } else {
            None
        };

        Ok(Self {
            entity,
            name,
            x,
            y,
            z,
            rotation,
            pitch,
            current_item,
            metadata,
        })
    }
}
