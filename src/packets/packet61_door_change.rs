//! Officially named `Sound/Particle Effect Packet`

use crate::network::connection::Encryption;
use crate::packets::io::MinecraftReadExt;
use crate::packets::packet_trait::ServerPacket;
use crate::packets::types::sound_effect::SoundEffect;
use std::io::Error;
use tokio::io::BufReader;
use tokio::net::tcp::OwnedReadHalf;

/// This packet treat every sound AND particles effect
pub struct DoorChangePacket {
    pub sound_effect: SoundEffect,
    pub x: i32,
    pub y: u8,
    pub z: i32,
    /// Extra data. Can be for exemple, when a potion explode, the extra data is the color of the potion
    pub aux_data: i32,
    /// False: Local sound, decrease with the distance
    /// True: Everyone hear it, everywhere. Used for boss
    pub broadcast: bool,
}

impl ServerPacket for DoorChangePacket {
    async fn read(
        reader: &mut BufReader<OwnedReadHalf>,
        encryption: &mut Encryption,
    ) -> Result<Self, Error>
    where
        Self: Sized,
    {
        Ok(Self {
            sound_effect: SoundEffect::from_id(reader.read_i32(encryption).await?),
            x: reader.read_i32(encryption).await?,
            y: reader.read_u8(encryption).await?,
            z: reader.read_i32(encryption).await?,
            aux_data: reader.read_i32(encryption).await?,
            broadcast: reader.read_u8(encryption).await? != 0,
        })
    }
}
