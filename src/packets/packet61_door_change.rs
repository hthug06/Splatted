//! Officially named `Sound/Particle Effect Packet`

use crate::network::connection::Encryption;
use crate::packets::io::MinecraftReadExt;
use crate::packets::packet_trait::ServerPacket;
use crate::packets::types::sound_effect::SoundEffect;
use crate::protocol_version::ProtocolVersion;
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
    /// Implemented in 1.4
    pub broadcast: Option<bool>,
}

impl ServerPacket for DoorChangePacket {
    async fn read(
        reader: &mut BufReader<OwnedReadHalf>,
        encryption: &mut Encryption,
        protocol_version: ProtocolVersion,
    ) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let sound_effect = SoundEffect::from_id(reader.read_i32(encryption).await?);
        let x = reader.read_i32(encryption).await?;
        let y = reader.read_u8(encryption).await?;
        let z = reader.read_i32(encryption).await?;
        let aux_data = reader.read_i32(encryption).await?;

        // For 1.4
        let broadcast = if protocol_version == ProtocolVersion::V1_4 {
            Some(reader.read_u8(encryption).await? != 0)
        }
        // For 1.2 and 1.3
        else {
            None
        };

        Ok(Self {
            sound_effect,
            x,
            y,
            z,
            aux_data,
            broadcast,
        })
    }
}
