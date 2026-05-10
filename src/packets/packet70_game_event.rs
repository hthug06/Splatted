use crate::network::connection::Encryption;
use crate::packets::io::MinecraftReadExt;
use crate::packets::packet_trait::ServerPacket;
use crate::packets::types::event_type::EventType;
use crate::packets::types::game_type::GameType;
use std::io::Error;
use tokio::io::BufReader;
use tokio::net::tcp::OwnedReadHalf;

pub struct GameEventPacket {
    pub event_type: EventType,
    pub game_mode: GameType,
}

impl ServerPacket for GameEventPacket {
    async fn read(
        reader: &mut BufReader<OwnedReadHalf>,
        encryption: &mut Encryption,
    ) -> Result<Self, Error>
    where
        Self: Sized,
    {
        Ok(Self {
            event_type: EventType::from_id(reader.read_i8(encryption).await?),
            game_mode: GameType::from_id(reader.read_i8(encryption).await?)
                .unwrap_or(GameType::Survival),
        })
    }
}
