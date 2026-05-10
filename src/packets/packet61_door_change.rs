//! Officially named `Sound/Particle Effect Packet`

use crate::network::connection::Encryption;
use crate::packets::packet_trait::ServerPacket;
use crate::packets::types::sound_effect::SoundEffect;
use crate::packets::utils::{read_i32, read_u8};
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
            sound_effect: SoundEffect::from_id(read_i32(reader, encryption).await?),
            x: read_i32(reader, encryption).await?,
            y: read_u8(reader, encryption).await?,
            z: read_i32(reader, encryption).await?,
            aux_data: read_i32(reader, encryption).await?,
            broadcast: read_u8(reader, encryption).await? != 0,
        })
    }
}
