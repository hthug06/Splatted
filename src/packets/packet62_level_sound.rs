use crate::network::connection::Encryption;
use crate::packets::io::MinecraftReadExt;
use crate::packets::packet_trait::ServerPacket;
use crate::protocol_version::ProtocolVersion;
use std::io::Error;
use tokio::io::BufReader;
use tokio::net::tcp::OwnedReadHalf;

pub struct LevelSoundPacket {
    pub sound_name: String,
    pub effect_x: i32,
    pub effect_y: i32,
    pub effect_z: i32,
    /// 1 is 100%. Can be more.
    pub volume: f32,
    /// 63 is 100%. Can be more.
    pub pitch: u8,
}

impl ServerPacket for LevelSoundPacket {
    async fn read(
        reader: &mut BufReader<OwnedReadHalf>,
        encryption: &mut Encryption,
        _protocol_version: ProtocolVersion,
    ) -> Result<Self, Error>
    where
        Self: Sized,
    {
        Ok(Self {
            sound_name: reader.read_string(encryption).await?,
            effect_x: reader.read_i32(encryption).await?,
            effect_y: reader.read_i32(encryption).await?,
            effect_z: reader.read_i32(encryption).await?,
            volume: reader.read_f32(encryption).await?,
            pitch: reader.read_u8(encryption).await?,
        })
    }
}
