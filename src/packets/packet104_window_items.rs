use crate::network::connection::Encryption;
use crate::packets::packet_trait::ServerPacket;
use crate::packets::types::itemstack::ItemStack;
use crate::packets::utils::{read_i16, read_u8};
use std::io::Error;
use tokio::io::BufReader;
use tokio::net::tcp::OwnedReadHalf;

#[derive(Debug)]
pub struct WindowItemsPacket {
    pub window_id: u8,
    pub slots: Vec<Option<ItemStack>>,
}

impl ServerPacket for WindowItemsPacket {
    async fn read(
        reader: &mut BufReader<OwnedReadHalf>,
        encryption: &mut Encryption,
    ) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let window_id = read_u8(reader, encryption).await?;

        // The number of itemstack
        let number_of_item = read_i16(reader, encryption).await?;

        // Read all the itemStack
        // Normally, it's 0 or > 0 item, but we need to be sure it's not a malformed | malicious packet
        let itemstacks: Vec<Option<ItemStack>> = if number_of_item >= 0 {
            let mut itemstacks: Vec<Option<ItemStack>> =
                Vec::with_capacity(number_of_item as usize);
            for _ in 0..number_of_item {
                itemstacks.push(ItemStack::read(reader, encryption).await?);
            }

            itemstacks
        } else {
            // 45 = number of slot in a Minecraft inventory
            vec![None; 45]
        };

        Ok(Self {
            window_id,
            slots: itemstacks,
        })
    }
}
