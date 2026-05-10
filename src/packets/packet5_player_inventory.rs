use crate::network::connection::Encryption;
use crate::packets::io::MinecraftReadExt;
use crate::packets::packet_trait::ServerPacket;
use crate::packets::types::itemstack::ItemStack;
use std::io::Error;
use tokio::io::BufReader;
use tokio::net::tcp::OwnedReadHalf;

pub struct PlayerInventoryPacket {
    pub entity_id: i32,
    pub slot: i16,
    pub item: Option<ItemStack>,
}

impl ServerPacket for PlayerInventoryPacket {
    async fn read(
        reader: &mut BufReader<OwnedReadHalf>,
        encryption: &mut Encryption,
    ) -> Result<Self, Error>
    where
        Self: Sized,
    {
        Ok(Self {
            entity_id: reader.read_i32(encryption).await?,
            slot: reader.read_i16(encryption).await?,
            item: ItemStack::read(reader, encryption).await?,
        })
    }
}
