use crate::network::connection::Encryption;
use crate::packets::io::MinecraftReadExt;
use crate::packets::packet_trait::ServerPacket;
use crate::protocol_version::ProtocolVersion;
use std::io::Error;
use tokio::io::BufReader;
use tokio::net::tcp::OwnedReadHalf;

pub struct PlayerAbilitiesPacket {
    pub disable_damage: bool,
    pub is_flying: bool,
    pub allow_flying: bool,
    pub creative_mode: bool,
    /// From 1.4
    pub fly_speed: Option<f32>,
    /// From 1.4
    pub walk_speed: Option<f32>,
}

impl ServerPacket for PlayerAbilitiesPacket {
    async fn read(
        reader: &mut BufReader<OwnedReadHalf>,
        encryption: &mut Encryption,
        protocol_version: ProtocolVersion,
    ) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let (disable_damage, is_flying, allow_flying, creative_mode, fly_speed, walk_speed) =
            // In 1.2, each variable is a byte
            if protocol_version == ProtocolVersion::V1_2 {
                let disable_damage: bool = reader.read_u8(encryption).await? != 0;
                let is_flying: bool = reader.read_u8(encryption).await? != 0;
                let allow_flying: bool = reader.read_u8(encryption).await? != 0;
                let creative_mode: bool = reader.read_u8(encryption).await? != 0;

                (disable_damage, is_flying, allow_flying, creative_mode, None, None)
            }
            // But from 1.3, It's more optimized
            else if protocol_version == ProtocolVersion::V1_3
                || protocol_version == ProtocolVersion::V1_4
                || protocol_version == ProtocolVersion::V1_5
                || protocol_version == ProtocolVersion::V1_6
            {
                let abilities_byte: u8 = reader.read_u8(encryption).await?;

                let disable_damage = (abilities_byte & 1) > 0;
                let is_flying = (abilities_byte & 2) > 0;
                let allow_flying = (abilities_byte & 4) > 0;
                let creative_mode = (abilities_byte & 8) > 0;



                let (fly_speed, walk_speed) = if protocol_version == ProtocolVersion::V1_6 {
                    // In 1.6, Mojang now use 'Float' (f32) to read these values and be more precise !
                    let fly_speed: f32 = reader.read_f32(encryption).await?;
                    let walk_speed: f32 = reader.read_f32(encryption).await?;
                    (fly_speed, walk_speed)
                } else {
                    // These value are not precise
                    // for exemple, 0.05 for the server is 0.047058824 for us
                    // Mojang change these later, but for now, we need to keep it
                    // (Also it cause desync sometime, but it's okay lol)
                    let fly_speed: f32 = (reader.read_u8(encryption).await? as f32) / 255.0;
                    let walk_speed: f32 = (reader.read_u8(encryption).await? as f32) / 255.0;
                    (fly_speed, walk_speed)
                };

                (disable_damage, is_flying, allow_flying, creative_mode, Some(fly_speed), Some(walk_speed))
            }
            // Everything to false for an unknown version
            else {
                (false, false, false, false, None, None)
            };

        Ok(Self {
            disable_damage,
            is_flying,
            allow_flying,
            creative_mode,
            fly_speed,
            walk_speed,
        })
    }
}
