use crate::network::connection::Encryption;
use crate::packets::io::MinecraftReadExt;
use crate::packets::packet_trait::ServerPacket;
use crate::packets::types::nbt_tag_compound::NbtTagCompound;
use crate::packets::types::tile_entity_action::TileEntityAction;
use crate::protocol_version::ProtocolVersion;
use std::io::Error;
use tokio::io::BufReader;
use tokio::net::tcp::OwnedReadHalf;

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
        _protocol_version: ProtocolVersion,
    ) -> Result<Self, Error>
    where
        Self: Sized,
    {
        Ok(Self {
            x: reader.read_i32(encryption).await?,
            y: reader.read_i16(encryption).await?,
            z: reader.read_i32(encryption).await?,
            action_type: TileEntityAction::from_id(reader.read_u8(encryption).await?),
            custom_param: NbtTagCompound::read(reader, encryption).await?,
        })
    }
}
