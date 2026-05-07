use crate::network::connection::Encryption;
use crate::packets::types::nbt_tag_compound::NbtTagCompound;
use crate::packets::utils::{read_i8, read_i16};
use std::io::Error;
use tokio::io::BufReader;
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
    pub nbt_tag_compound: NbtTagCompound,
}

impl ItemStack {
    pub async fn read(
        reader: &mut BufReader<OwnedReadHalf>,
        encryption: &mut Encryption,
    ) -> Result<Option<Self>, Error> {
        // Read all the itemstack value (id, stack size and item damage)

        // First the id
        let id = read_i16(reader, encryption).await?;

        // an id < 0 can't exist, or maybe it's just air ?
        if id < 0 {
            return Ok(None);
        }

        // The stack size
        let stack_size = read_i8(reader, encryption).await?;

        // The item damage
        let item_damage = read_i16(reader, encryption).await?;

        // Read the NBT Tag Compound
        let nbt_tag_compound = NbtTagCompound::read(reader, encryption).await?;

        Ok(Some(Self {
            id,
            stack_size,
            item_damage,
            nbt_tag_compound,
        }))
    }
}
