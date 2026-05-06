use crate::network::connection::Encryption;
use crate::packets::utils::{read_i8, read_i16};
use std::io::Error;
use tokio::io::{AsyncReadExt, BufReader};
use tokio::net::tcp::OwnedReadHalf;

#[derive(Debug)]
/// An ItemStack is a representation of in Item ingame with the id, number of item and NBT
pub struct ItemStack {
    /// Max 64 so u8, but we parse it from TCP, so we need to respect the read type
    pub id: i16,
    /// I remember seen negative item with glitch, I hope this will be enough to parse it...
    /// Even if, we check if the value is > 0 in read_itemstack
    pub stack_size: i8,
    /// Used for the durability, wool | glass color...
    pub item_damage: i16,
    pub nbt_tag_compound: Option<Vec<u8>>,
}

impl ItemStack {
    pub async fn read_itemstack(
        reader: &mut BufReader<OwnedReadHalf>,
        encryption: &mut Encryption,
    ) -> Result<Option<Self>, Error> {
        // Read all the itemstack value (id, stack size and item damage)
        let id = read_i16(reader, encryption).await?;

        if id < 0 {
            return Ok(None);
        }

        let stack_size = read_i8(reader, encryption).await?;
        let item_damage = read_i16(reader, encryption).await?;

        // Read the NBT Tag Compound
        // First the size
        let nbt_length = read_i16(reader, encryption).await?;

        // Then, check if the ItemStack have some nbt data
        // The data is compressed in GZIP.
        let nbt_tag_compound = if nbt_length > 0 {
            // Create a vec the size of the data
            let mut nbt_bytes = vec![0u8; nbt_length as usize];

            // read the data (put it into the vec)
            reader.read_exact(&mut nbt_bytes).await?;

            // IMPORTANT: decrypt
            encryption.decrypt(&mut nbt_bytes);

            Some(nbt_bytes)
        } else {
            None
        };

        Ok(Some(Self {
            id,
            stack_size,
            item_damage,
            nbt_tag_compound,
        }))
    }
}
