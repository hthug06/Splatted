use crate::network::connection::Encryption;
use crate::packets::io::MinecraftReadExt;
use crate::packets::packet_trait::ServerPacket;
use crate::packets::packet30_entity::EntityPacket;
use crate::packets::types::entity_metadata::EntityMetadata;
use crate::packets::types::entity_type::EntityType;
use crate::protocol_version::ProtocolVersion;
use std::io::Error;
use tokio::io::BufReader;
use tokio::net::tcp::OwnedReadHalf;

pub struct MobSpawnPacket {
    entity_id: EntityPacket,
    entity_type: EntityType,
    x: i32,
    y: i32,
    z: i32,
    yaw: i8,
    pitch: i8,
    head_yaw: i8,
    /// Implemented in 1.3
    velocity_x: Option<i16>,
    /// Implemented in 1.3
    velocity_y: Option<i16>,
    /// Implemented in 1.3
    velocity_z: Option<i16>,
    metadata: EntityMetadata,
}

impl ServerPacket for MobSpawnPacket {
    async fn read(
        reader: &mut BufReader<OwnedReadHalf>,
        encryption: &mut Encryption,
        protocol_version: ProtocolVersion,
    ) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let entity_id = EntityPacket::read(reader, encryption, protocol_version).await?;
        // In mc code, they use & 255. Because we have unsigned integer, this is useless
        let entity_type = EntityType::from_id(reader.read_u8(encryption).await?);
        let x = (reader.read_i32(encryption).await?) / 32; // In src code, the /32, is divided by 32 and rounded
        let y = (reader.read_i32(encryption).await?) / 32; // So if we want to be precise later, we need to
        let z = (reader.read_i32(encryption).await?) / 32; // cast as f64 and divide / 32.0
        let yaw = reader.read_i8(encryption).await?;
        let pitch = reader.read_i8(encryption).await?;
        let head_yaw = reader.read_i8(encryption).await?;

        // Implemented in 1.3
        let (velocity_x, velocity_y, velocity_z) = if protocol_version == ProtocolVersion::V1_3
            || protocol_version == ProtocolVersion::V1_4
            || protocol_version == ProtocolVersion::V1_5
        {
            let velocity_x = reader.read_i16(encryption).await?;
            let velocity_y = reader.read_i16(encryption).await?;
            let velocity_z = reader.read_i16(encryption).await?;

            (Some(velocity_x), Some(velocity_y), Some(velocity_z))
        } else {
            (None, None, None)
        };

        Ok(Self {
            entity_id,
            entity_type,
            x,
            y,
            z,
            yaw,
            pitch,
            head_yaw,
            velocity_x,
            velocity_y,
            velocity_z,
            metadata: EntityMetadata::read(reader, encryption, protocol_version).await?,
        })
    }
}
