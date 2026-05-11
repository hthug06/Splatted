use crate::network::connection::Encryption;
use crate::packets::io::MinecraftReadExt;
use crate::packets::packet_trait::ServerPacket;
use crate::packets::packet30_entity::EntityPacket;
use crate::packets::types::entity_type::EntityType;
use std::io::Error;
use tokio::io::BufReader;
use tokio::net::tcp::OwnedReadHalf;

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
        let entity_type = EntityType::from_id(reader.read_u8(encryption).await?);
        let x = reader.read_i32(encryption).await?;
        let y = reader.read_i32(encryption).await?;
        let z = reader.read_i32(encryption).await?;
        let yaw = reader.read_i8(encryption).await?;
        let pitch = reader.read_i8(encryption).await?;
        let thrower_entity = EntityPacket::read(reader, encryption).await?;

        let (speed_x, speed_y, speed_z) = if thrower_entity.entity_id > 0 {
            (
                Some(reader.read_i16(encryption).await?),
                Some(reader.read_i16(encryption).await?),
                Some(reader.read_i16(encryption).await?),
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
