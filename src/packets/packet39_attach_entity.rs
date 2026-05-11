use crate::network::connection::Encryption;
use crate::packets::packet_trait::ServerPacket;
use crate::packets::packet30_entity::EntityPacket;
use crate::protocol_version::ProtocolVersion;
use std::io::Error;
use tokio::io::BufReader;
use tokio::net::tcp::OwnedReadHalf;

/// Used when an entity mounts or dismounts a vehicle (like a minecart, boat, or horse).
pub struct AttachEntityPacket {
    /// The entity who mount the vehicle
    pub entity: EntityPacket,
    /// Some = get into the vehicle | None = get out of the vehicle
    /// Also this is an entity id
    pub vehicle_entity: Option<EntityPacket>,
}

impl ServerPacket for AttachEntityPacket {
    async fn read(
        reader: &mut BufReader<OwnedReadHalf>,
        encryption: &mut Encryption,
        protocol_version: ProtocolVersion,
    ) -> Result<Self, Error> {
        let entity = EntityPacket::read(reader, encryption, protocol_version).await?;
        let raw_vehicle_entity = EntityPacket::read(reader, encryption, protocol_version).await?;

        // If this is -1, the player get out of the vehicle
        let vehicle_entity = if raw_vehicle_entity.entity_id == -1 {
            // Get out of the vehicle
            None
        } else {
            // Get in the vehicle
            Some(raw_vehicle_entity)
        };

        Ok(Self {
            entity,
            vehicle_entity,
        })
    }
}
