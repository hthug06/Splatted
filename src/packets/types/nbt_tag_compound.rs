use crate::network::connection::Encryption;
use crate::packets::io::MinecraftReadExt;
use std::io::Error;
use tokio::io::{AsyncReadExt, BufReader};
use tokio::net::tcp::OwnedReadHalf;

#[derive(Clone)]
pub struct NbtTagCompound {
    data: Option<Vec<u8>>,
}

impl NbtTagCompound {
    /// Create an empty nbt (for before the 1.4)
    pub fn empty() -> Self {
        Self { data: None }
    }

    /// Read an NBT from the buffer
    pub async fn read(
        reader: &mut BufReader<OwnedReadHalf>,
        encryption: &mut Encryption,
    ) -> Result<Self, Error> {
        // First the size
        let nbt_length = MinecraftReadExt::read_i16(reader, encryption).await?;

        // Then, check if the ItemStack have some nbt data
        if nbt_length <= 0 {
            return Ok(Self { data: None });
        }

        // Create a vec the size of the data.
        // The data is compressed in GZIP.
        let mut nbt_bytes = vec![0u8; nbt_length as usize];

        // read the data (put it into the vec)
        reader.read_exact(&mut nbt_bytes).await?;

        // IMPORTANT: decrypt
        encryption.decrypt(&mut nbt_bytes);

        Ok(Self {
            data: Some(nbt_bytes),
        })
    }
}
