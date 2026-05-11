use crate::network::connection::Encryption;
use crate::packets::io::MinecraftReadExt;
use crate::packets::packet_trait::ServerPacket;
use crate::packets::types::dimension_type::DimensionType;
use crate::packets::types::game_type::GameType;
use crate::packets::types::world_type::WorldType;
use crate::protocol_version::ProtocolVersion;
use std::io::Error;
use tokio::io::BufReader;
use tokio::net::tcp::OwnedReadHalf;

pub struct LoginPacket {
    /// For 1.4.7
    client_id: Option<i32>,
    /// For 1.2
    protocol_version: Option<i32>,
    username: Option<String>,
    terrain_type: WorldType,
    /// true = server in hardcore mode
    /// Not in 1.2
    hardcore: Option<bool>,
    game_type: GameType,
    /// -1: The Nether, 0: The Overworld, 1: The End
    dimension: DimensionType,
    /// 0: Peaceful, 1: Easy, 2: Normal, 3: Hard
    difficulty: i8,
    /// not used in 1.4.7, but need to be parsed for 1.2
    world_height: u8,
    /// not used in 1.4.7, but need to be parsed for 1.2
    max_players: u8,
}

impl ServerPacket for LoginPacket {
    /// Read the packet from the stream IN ORDER (this is important)
    /// Source are from here: https://www.a-centauri.com/archivio/1.4.7/ForgeDOC/src-html/net/minecraft/network/packet/Packet1Login.html#line.11
    async fn read(
        reader: &mut BufReader<OwnedReadHalf>,
        encryption: &mut Encryption,
        protocol_version: ProtocolVersion,
    ) -> Result<Self, Error>
    where
        Self: Sized,
    {
        // 1.2 only
        let (packet_protocol_version, username) = if protocol_version == ProtocolVersion::V1_2 {
            let packet_protocol_version = reader.read_i32(encryption).await?;
            let username = reader.read_string(encryption).await?;
            (Some(packet_protocol_version), Some(username))
        } else {
            (None, None)
        };

        // 1.4.7 only
        let client_id = if protocol_version == ProtocolVersion::V1_4 {
            Some(reader.read_i32(encryption).await?)
        } else {
            None
        };

        // Common
        let terrain_type_string = reader.read_string(encryption).await?;
        let terrain_type = WorldType::parse(&terrain_type_string);

        // 1.4.7: hardcore + game_type packed in one byte
        let (hardcore, game_type, dimension_id) = if protocol_version == ProtocolVersion::V1_4 {
            // Little trick from the forge source code to save bandwidth
            let byte = reader.read_i8(encryption).await?;
            let hardcore = (byte & 8) != 0; // bit 3
            let game_type = GameType::from_id(byte & 7);
            let dimension_id = reader.read_i8(encryption).await?;
            (Some(hardcore), game_type, dimension_id)
        }
        // 1.2:   game_type as standalone i32
        else if protocol_version == ProtocolVersion::V1_2 {
            let game_type = GameType::from_id(reader.read_i32(encryption).await? as i8);
            let dimension_id = reader.read_i32(encryption).await? as i8;
            (None, game_type, dimension_id)
        }
        // Other Version
        else {
            (None, GameType::Survival, 0)
        };

        let dimension = DimensionType::from_id(dimension_id);
        let difficulty = reader.read_i8(encryption).await?;
        let world_height = reader.read_u8(encryption).await?; // Need to parse it for 1.2
        let max_players = reader.read_u8(encryption).await?; // no the max player is not 127 lol

        Ok(Self {
            client_id,
            protocol_version: packet_protocol_version,
            username,
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
