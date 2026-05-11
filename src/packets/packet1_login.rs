use crate::network::connection::Encryption;
use crate::packets::io::{MinecraftReadExt, MinecraftWriteExt};
use crate::packets::packet_trait::{ClientPacket, ServerPacket};
use crate::packets::types::dimension_type::DimensionType;
use crate::packets::types::game_type::GameType;
use crate::packets::types::world_type::WorldType;
use crate::protocol_version::ProtocolVersion;
use bytes::{BufMut, BytesMut};
use std::io::Error;
use tokio::io::BufReader;
use tokio::net::tcp::OwnedReadHalf;

pub struct LoginPacket {
    /// For 1.4.7
    pub client_id: Option<i32>,
    /// For 1.2
    pub protocol_version: Option<i32>,
    pub username: Option<String>,
    pub terrain_type: WorldType,
    /// true = server in hardcore mode
    /// Not in 1.2
    pub hardcore: Option<bool>,
    pub game_type: GameType,
    /// -1: The Nether, 0: The Overworld, 1: The End
    pub dimension: DimensionType,
    /// 0: Peaceful, 1: Easy, 2: Normal, 3: Hard
    pub difficulty: i8,
    /// not used in 1.4.7, but need to be parsed for 1.2
    pub world_height: u8,
    /// not used in 1.4.7, but need to be parsed for 1.2
    pub max_players: u8,
}

impl Default for LoginPacket {
    fn default() -> Self {
        Self {
            // protocol version and username need to be exact
            protocol_version: Some(0),
            username: Some("".to_string()),

            // The rest, we need to send it but it's useless
            terrain_type: WorldType::Default,
            game_type: GameType::Survival,
            dimension: DimensionType::Overworld,
            difficulty: 0,
            world_height: 0,
            max_players: 0,
            client_id: None,
            hardcore: None,
        }
    }
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

impl ClientPacket for LoginPacket {
    fn write_to(
        &self,
        buffer: &mut BytesMut,
        _protocol_version: ProtocolVersion,
    ) -> Result<(), Error> {
        // ID of the packet
        buffer.put_u8(0x01);

        // The packet is only received on 1.2, so it work 100%
        let protocol_version = self.protocol_version.ok_or_else(|| {
            Error::new(
                std::io::ErrorKind::InvalidData,
                "Protocol version is required for LoginPacket (not the right version)",
            )
        })?;

        //The packet is send in 1.2
        match ProtocolVersion::from_protocol_version(protocol_version as u32) {
            ProtocolVersion::V1_2 => {
                buffer.put_i32(protocol_version);
                buffer.write_string(&self.username.clone().unwrap())?;

                // Useless data
                buffer.write_string(&self.terrain_type.name())?; // Terrain Type
                buffer.put_i32(self.game_type.id()); // Server Mode
                buffer.put_i32(self.dimension.id() as i32); // Dimension
                buffer.put_i8(self.difficulty); // Difficulty
                buffer.put_u8(self.world_height); // World Height
                buffer.put_u8(self.max_players); // Max Players
            }
            _ => {
                // The packet is never send in 1.4
                // Instead, the client status packet (ClientCommandPacket 205)
            }
        }

        Ok(())
    }
}
