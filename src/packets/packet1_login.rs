use crate::network::connection::Encryption;
use crate::packets::packet_trait::ServerPacket;
use crate::packets::types::dimension_type::DimensionType;
use crate::packets::types::game_type::GameType;
use crate::packets::types::world_type::WorldType;
use crate::packets::utils::{read_i8, read_i32, read_string};
use std::io::Error;
use tokio::io::BufReader;
use tokio::net::tcp::OwnedReadHalf;

#[derive(Debug)]
pub struct LoginPacket {
    client_id: i32,
    terrain_type: WorldType,
    /// true = server in hardcore mode
    hardcore: bool,
    game_type: GameType,
    /// -1: The Nether, 0: The Overworld, 1: The End
    dimension: DimensionType,
    /// 0: Peaceful, 1: Easy, 2: Normal, 3: Hard
    difficulty: i8,
    /// not used in 1.4.7, but need to be parsed
    world_height: i8,
    /// not used in 1.4.7, but need to be parsed
    max_players: i8,
}

impl ServerPacket for LoginPacket {
    /// Read the packet from the stream IN ORDER (this is important)
    /// Source are from here: https://www.a-centauri.com/archivio/1.4.7/ForgeDOC/src-html/net/minecraft/network/packet/Packet1Login.html#line.11
    async fn read(
        reader: &mut BufReader<OwnedReadHalf>,
        encryption: &mut Encryption,
    ) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let client_id = read_i32(reader, encryption).await?;

        let terrain_type_string = read_string(reader, encryption).await?;
        let terrain_type = WorldType::parse(&terrain_type_string);

        let hardcore_and_game_type_byte = read_i8(reader, encryption).await?;
        let hardcore = (hardcore_and_game_type_byte) == 8;

        // Little trick from the forge source code to save bandwidth (yes)
        let game_type_id = hardcore_and_game_type_byte & 7;
        let game_type = GameType::from_id(game_type_id).unwrap_or(GameType::Survival);

        let dimension_id = read_i8(reader, encryption).await?;
        let dimension = DimensionType::from_id(dimension_id);

        let difficulty = read_i8(reader, encryption).await?;
        let world_height = read_i8(reader, encryption).await?; // useless now but we need to parse it...
        let max_players = read_i8(reader, encryption).await?; // no the max player is not 255 lol

        Ok(Self {
            client_id,
            terrain_type,
            hardcore,
            game_type,
            dimension,
            difficulty,
            world_height,
            max_players,
        })
    }
}
