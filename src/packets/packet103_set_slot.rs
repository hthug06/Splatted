use crate::network::connection::Encryption;
use crate::packets::packet_trait::ServerPacket;
use crate::packets::types::itemstack::ItemStack;
use crate::packets::utils::{read_i8, read_i16};
use std::io::Error;
use tokio::io::BufReader;
use tokio::net::tcp::OwnedReadHalf;

#[derive(Debug)]
pub struct SetSlotPacket {
    // why i8? because the first SetSlot packet send = SetSlotPacket { windows_id: -1, slot: -1, item_stack: None } ????
    pub windows_id: i8,
    pub slot: i16,
    pub item_stack: Option<ItemStack>,
}

impl ServerPacket for SetSlotPacket {
    async fn read(
        reader: &mut BufReader<OwnedReadHalf>,
        encryption: &mut Encryption,
    ) -> Result<Self, Error>
    where
        Self: Sized,
    {
        Ok(Self {
            windows_id: read_i8(reader, encryption).await?,
            slot: read_i16(reader, encryption).await?,
            // Normally, if this set slot the itemstack will be 100% there
            // But just to be sure (and because I don't want any crash), let's have an Option<ItemStack>
            item_stack: ItemStack::read(reader, encryption).await?,
        })
    }
}
