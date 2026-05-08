use crate::network::connection::Encryption;
use crate::packets::packet_trait::ServerPacket;
use crate::packets::types::entity_metadata::EntityMetadata;
use crate::packets::types::entity_type::EntityType;
use crate::packets::utils::{read_i8, read_i16, read_i32, read_u8};
use std::io::Error;
use tokio::io::BufReader;
use tokio::net::tcp::OwnedReadHalf;

#[derive(Debug)]
pub struct MobSpawnPacket {
    entity_id: i32,
    entity_type: EntityType,
    x: i32,
    y: i32,
    z: i32,
    yaw: i8,
    pitch: i8,
    head_yaw: i8,
    velocity_x: i16,
    velocity_y: i16,
    velocity_z: i16,
    metadata: EntityMetadata,
}

impl ServerPacket for MobSpawnPacket {
    async fn read(
        reader: &mut BufReader<OwnedReadHalf>,
        encryption: &mut Encryption,
    ) -> Result<Self, Error>
    where
        Self: Sized,
    {
        Ok(Self {
            entity_id: read_i32(reader, encryption).await?,
            // In mc code, they use & 255. Because we have unsigned integer, this is useless
            entity_type: EntityType::from_id(read_u8(reader, encryption).await?),
            x: (read_i32(reader, encryption).await?) / 32, // In src code, the /32, is divided by 32 and rounded
            y: (read_i32(reader, encryption).await?) / 32, // So if we want to be precise later, we need to
            z: (read_i32(reader, encryption).await?) / 32, // cast as f64 and divide / 32.0
            yaw: read_i8(reader, encryption).await?,
            pitch: read_i8(reader, encryption).await?,
            head_yaw: read_i8(reader, encryption).await?,
            velocity_x: read_i16(reader, encryption).await?,
            velocity_y: read_i16(reader, encryption).await?,
            velocity_z: read_i16(reader, encryption).await?,
            metadata: EntityMetadata::read(reader, encryption).await?,
        })
    }
}
