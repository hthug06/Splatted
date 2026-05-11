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
    /// Only for 1.4
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
        let packet = Self {
            entity: EntityPacket::read(reader, encryption, protocol_version).await?,
            name: reader.read_string(encryption).await?,
            x: reader.read_i32(encryption).await?,
            y: reader.read_i32(encryption).await?,
            z: reader.read_i32(encryption).await?,
            rotation: reader.read_i8(encryption).await?,
            pitch: reader.read_i8(encryption).await?,
            current_item: ItemStack::read(reader, encryption).await?,
            metadata: None,
        };

        let metadata = if protocol_version == ProtocolVersion::V1_4 {
            Some(EntityMetadata::read(reader, encryption, protocol_version).await?)
        } else {
            None
        };

        Ok(Self { metadata, ..packet })
    }
}
