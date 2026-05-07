use crate::network::connection::Encryption;
use crate::packets::packet_trait::ServerPacket;
use crate::packets::utils::{read_f32, read_i32, read_string, read_u8};
use std::io::Error;
use tokio::io::BufReader;
use tokio::net::tcp::OwnedReadHalf;

#[derive(Debug)]
pub struct LevelSoundPacket {
    pub sound_name: String,
    pub effect_x: i32,
    pub effect_y: i32,
    pub effect_z: i32,
    pub volume: f32,
    pub pitch: u8,
}

impl ServerPacket for LevelSoundPacket {
    async fn read(
        reader: &mut BufReader<OwnedReadHalf>,
        encryption: &mut Encryption,
    ) -> Result<Self, Error>
    where
        Self: Sized,
    {
        Ok(Self {
            sound_name: read_string(reader, encryption).await?,
            effect_x: read_i32(reader, encryption).await?,
            effect_y: read_i32(reader, encryption).await?,
            effect_z: read_i32(reader, encryption).await?,
            volume: read_f32(reader, encryption).await?,
            pitch: read_u8(reader, encryption).await?,
        })
    }
}
