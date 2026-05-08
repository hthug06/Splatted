use crate::network::connection::Encryption;
use crate::packets::packet_trait::ServerPacket;
use crate::packets::packet30_entity::EntityPacket;
use std::io::Error;
use tokio::io::BufReader;
use tokio::net::tcp::OwnedReadHalf;

#[derive(Debug)]
/// Used when an entity mounts or dismounts a vehicle (like a minecart, boat, or horse).
pub struct AttachEntityPacket {
    /// The entity who mount the vehicule
    pub entity: EntityPacket,
    /// Some = get into the vehicule | None = get out of the vehicule
    /// Also this is an entity id
    pub vehicle_entity: Option<EntityPacket>,
}

impl ServerPacket for AttachEntityPacket {
    async fn read(
        reader: &mut BufReader<OwnedReadHalf>,
        encryption: &mut Encryption,
    ) -> Result<Self, Error> {
        let entity = EntityPacket::read(reader, encryption).await?;
        let raw_vehicle_entity = EntityPacket::read(reader, encryption).await?;

        // On transforme le hack de Notch en beau code Rust
        let vehicle_entity = if raw_vehicle_entity.entity_id == -1 {
            // Get out of the vehicule
            None
        } else {
            // Get in the vehicule
            Some(raw_vehicle_entity)
        };

        Ok(Self {
            entity,
            vehicle_entity,
        })
    }
}
