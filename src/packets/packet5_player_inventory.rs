use crate::network::connection::Encryption;
use crate::packets::packet_trait::ServerPacket;
use crate::packets::types::itemstack::ItemStack;
use crate::packets::utils::{read_i16, read_i32};
use std::io::Error;
use tokio::io::BufReader;
use tokio::net::tcp::OwnedReadHalf;

#[derive(Debug)]
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
            entity_id: read_i32(reader, encryption).await?,
            slot: read_i16(reader, encryption).await?,
            item: ItemStack::read(reader, encryption).await?,
        })
    }
}
