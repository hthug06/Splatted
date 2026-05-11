use crate::network::connection::Encryption;
use crate::packets::io::MinecraftReadExt;
use crate::packets::packet_trait::ServerPacket;
use crate::packets::packet30_entity::EntityPacket;
use crate::protocol_version::ProtocolVersion;
use std::io::Error;
use tokio::io::BufReader;
use tokio::net::tcp::OwnedReadHalf;

pub struct DestroyEntityPacket {
    pub entity_ids: Vec<EntityPacket>,
}

impl ServerPacket for DestroyEntityPacket {
    async fn read(
        reader: &mut BufReader<OwnedReadHalf>,
        encryption: &mut Encryption,
        protocol_version: ProtocolVersion,
    ) -> Result<Self, Error>
    where
        Self: Sized,
    {
        // 1.2
        if protocol_version == ProtocolVersion::V1_2 {
            return Ok(Self {
                entity_ids: vec![EntityPacket::read(reader, encryption, protocol_version).await?],
            });
        }

        // in 1.4, it's more optimized
        // Here, instead of sending 10 packet to destroy 10 entities, the server can send only 1 packet
        // With all the entities to destroy
        let entity_count = reader.read_u8(encryption).await?;

        // Then, we can parse All entities id into a Vec
        // We can create a vec with a defined capacitiy because of the count variable
        let mut entity_ids = Vec::with_capacity(entity_count as usize);

        for _ in 0..entity_count {
            entity_ids.push(EntityPacket::read(reader, encryption, protocol_version).await?);
        }

        Ok(Self { entity_ids })
    }
}
