use crate::network::connection::Encryption;
use crate::packets::io::MinecraftReadExt;
use crate::packets::packet_trait::ServerPacket;
use crate::packets::packet30_entity::EntityPacket;
use crate::packets::types::itemstack::ItemStack;
use crate::protocol_version::ProtocolVersion;
use std::io::Error;
use tokio::io::BufReader;
use tokio::net::tcp::OwnedReadHalf;

pub struct PlayerInventoryPacket {
    /// Entity of the object.
    pub entity: EntityPacket,
    /// Equipment slot: 0=held, 1-4=armor slot
    pub slot: i16,
    pub item: Option<ItemStack>,
}

impl ServerPacket for PlayerInventoryPacket {
    async fn read(
        reader: &mut BufReader<OwnedReadHalf>,
        encryption: &mut Encryption,
        protocol_version: ProtocolVersion,
    ) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let entity = EntityPacket::read(reader, encryption, protocol_version).await?;
        let slot = reader.read_i16(encryption).await?;
        let item = if protocol_version == ProtocolVersion::V1_3
            || protocol_version == ProtocolVersion::V1_4
        {
            // In 1.3, they created a function to parse the ItemStack
            ItemStack::read(reader, encryption).await?
        } else if protocol_version == ProtocolVersion::V1_2 {
            // In 1.2, you get the itemId and the ItemDamage separately, and you have to create the ItemStack yourself
            let item_id = reader.read_i16(encryption).await?;
            let item_damage = reader.read_i16(encryption).await?;

            ItemStack::new_simple(item_id, None, Some(item_damage))
        } else {
            None
        };

        Ok(Self { entity, slot, item })
    }
}
