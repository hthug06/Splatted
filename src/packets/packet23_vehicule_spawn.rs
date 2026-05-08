use crate::network::connection::Encryption;
use crate::packets::packet_trait::ServerPacket;
use crate::packets::packet30_entity::EntityPacket;
use crate::packets::types::entity_type::EntityType;
use crate::packets::utils::{read_i8, read_i16, read_i32, read_u8};
use std::io::Error;
use tokio::io::BufReader;
use tokio::net::tcp::OwnedReadHalf;

#[derive(Debug)]
pub struct VehicleSpawnPacket {
    pub entity: EntityPacket,
    pub entity_type: EntityType,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub yaw: i8,
    pub pitch: i8,
    pub thrower_entity: EntityPacket,
    pub speed_x: Option<i16>,
    pub speed_y: Option<i16>,
    pub speed_z: Option<i16>,
}

impl ServerPacket for VehicleSpawnPacket {
    async fn read(
        reader: &mut BufReader<OwnedReadHalf>,
        encryption: &mut Encryption,
    ) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let entity = EntityPacket::read(reader, encryption).await?;
        let entity_type = EntityType::from_id(read_u8(reader, encryption).await?);
        let x = read_i32(reader, encryption).await?;
        let y = read_i32(reader, encryption).await?;
        let z = read_i32(reader, encryption).await?;
        let yaw = read_i8(reader, encryption).await?;
        let pitch = read_i8(reader, encryption).await?;
        let thrower_entity = EntityPacket::read(reader, encryption).await?;

        let (speed_x, speed_y, speed_z) = if thrower_entity.entity_id > 0 {
            (
                Some(read_i16(reader, encryption).await?),
                Some(read_i16(reader, encryption).await?),
                Some(read_i16(reader, encryption).await?),
            )
        } else {
            (None, None, None)
        };

        Ok(Self {
            entity,
            entity_type,
            x,
            y,
            z,
            yaw,
            pitch,
            thrower_entity,
            speed_x,
            speed_y,
            speed_z,
        })
    }
}
