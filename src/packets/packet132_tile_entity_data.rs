use crate::network::connection::Encryption;
use crate::packets::packet_trait::ServerPacket;
use crate::packets::types::nbt_tag_compound::NbtTagCompound;
use crate::packets::utils::{read_i16, read_i32, read_u8};
use std::io::Error;
use tokio::io::BufReader;
use tokio::net::tcp::OwnedReadHalf;
use crate::packets::types::tile_entity_action::TileEntityAction;

#[derive(Debug)]
pub struct TileEntityDataPacket {
    pub x: i32,
    pub y: i16,
    pub z: i32,
    pub action_type: TileEntityAction,
    pub custom_param: NbtTagCompound,
}

impl ServerPacket for TileEntityDataPacket {
    async fn read(
        reader: &mut BufReader<OwnedReadHalf>,
        encryption: &mut Encryption,
    ) -> Result<Self, Error>
    where
        Self: Sized,
    {
        Ok(Self {
            x: read_i32(reader, encryption).await?,
            y: read_i16(reader, encryption).await?,
            z: read_i32(reader, encryption).await?,
            action_type: TileEntityAction::from_id(read_u8(reader, encryption).await?),
            custom_param: NbtTagCompound::read(reader, encryption).await?,
        })
    }
}
