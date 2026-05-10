use crate::network::connection::Encryption;
use crate::packets::packet_trait::ServerPacket;
use crate::packets::packet30_entity::EntityPacket;
use crate::packets::types::entity_metadata::EntityMetadata;
use crate::packets::types::itemstack::ItemStack;
use crate::packets::utils::{read_i8, read_i32, read_string};
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
    pub metadata: EntityMetadata,
}

impl ServerPacket for NamedEntitySpawnPacket {
    async fn read(
        reader: &mut BufReader<OwnedReadHalf>,
        encryption: &mut Encryption,
    ) -> Result<Self, Error>
    where
        Self: Sized,
    {
        Ok(Self {
            entity: EntityPacket::read(reader, encryption).await?,
            name: read_string(reader, encryption).await?,
            x: read_i32(reader, encryption).await?,
            y: read_i32(reader, encryption).await?,
            z: read_i32(reader, encryption).await?,
            rotation: read_i8(reader, encryption).await?,
            pitch: read_i8(reader, encryption).await?,
            current_item: ItemStack::read(reader, encryption).await?,
            metadata: EntityMetadata::read(reader, encryption).await?,
        })
    }
}
