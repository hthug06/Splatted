use crate::network::connection::Encryption;
use crate::packets::packet_trait::ServerPacket;
use crate::packets::utils::read_u8;
use std::io::Error;
use tokio::io::BufReader;
use tokio::net::tcp::OwnedReadHalf;

#[derive(Debug)]
pub struct PlayerAbilitiesPacket {
    pub disable_damage: bool,
    pub is_flying: bool,
    pub allow_flying: bool,
    pub creative_mode: bool,
    pub fly_speed: f32,
    pub walk_speed: f32,
}

impl ServerPacket for PlayerAbilitiesPacket {
    async fn read(
        reader: &mut BufReader<OwnedReadHalf>,
        encryption: &mut Encryption,
    ) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let abilities_byte: u8 = read_u8(reader, encryption).await?;

        let disable_damage = (abilities_byte & 1) > 0;
        let is_flying = (abilities_byte & 2) > 0;
        let allow_flying = (abilities_byte & 4) > 0;
        let creative_mode = (abilities_byte & 8) > 0;

        // These value are not precise
        // for exemple, 0.05 for the server is 0.047058824 for us
        // Mojang change these later, but for now, we need to keep it
        // (Also it cause desync sometime, but it's okay lol)
        let fly_speed: f32 = (read_u8(reader, encryption).await? as f32) / 255.0;
        let walk_speed: f32 = (read_u8(reader, encryption).await? as f32) / 255.0;

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
