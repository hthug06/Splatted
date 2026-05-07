use crate::network::connection::Encryption;
use crate::packets::utils::read_i16;
use std::io::Error;
use tokio::io::{AsyncReadExt, BufReader};
use tokio::net::tcp::OwnedReadHalf;

#[derive(Debug)]
pub struct NbtTagCompound {
    data: Vec<u8>,
}

impl NbtTagCompound {
    pub async fn read(
        reader: &mut BufReader<OwnedReadHalf>,
        encryption: &mut Encryption,
    ) -> Result<Option<Self>, Error> {
        // First the size
        let nbt_length = read_i16(reader, encryption).await?;

        // Then, check if the ItemStack have some nbt data
        if nbt_length <= 0 {
            return Ok(None);
        }

        // Create a vec the size of the data.
        // The data is compressed in GZIP.
        let mut nbt_bytes = vec![0u8; nbt_length as usize];

        // read the data (put it into the vec)
        reader.read_exact(&mut nbt_bytes).await?;

        // IMPORTANT: decrypt
        encryption.decrypt(&mut nbt_bytes);

        Ok(Some(Self { data: nbt_bytes }))
    }
}
