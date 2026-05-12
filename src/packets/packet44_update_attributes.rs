use crate::network::connection::Encryption;
use crate::packets::io::MinecraftReadExt;
use crate::packets::packet_trait::ServerPacket;
use crate::packets::packet30_entity::EntityPacket;
use crate::packets::types::attribute_key::AttributeKey;
use crate::protocol_version::ProtocolVersion;
use std::io::Error;
use tokio::io::BufReader;
use tokio::net::tcp::OwnedReadHalf;
use uuid::Uuid;

/// This packet update the attribute of an entity
pub struct UpdateAttributesPacket {
    pub entity: EntityPacket,
    pub properties: Vec<EntityProperty>,
}

/// This struct represent a property of an entity with his key, base value and modifiers
pub struct EntityProperty {
    pub key: AttributeKey,
    pub base_value: f64,
    pub modifiers: Vec<AttributeModifier>,
}

/// this struct represent an attribute modifier for an entity
pub struct AttributeModifier {
    pub uuid: Uuid,
    pub name: String,
    pub amount: f64,
    pub operation: i32,
}

impl ServerPacket for UpdateAttributesPacket {
    async fn read(
        reader: &mut BufReader<OwnedReadHalf>,
        encryption: &mut Encryption,
        protocol_version: ProtocolVersion,
    ) -> Result<Self, Error>
    where
        Self: Sized,
    {
        // The entity of the packet
        let entity = EntityPacket::read(reader, encryption, protocol_version).await?;
        let property_count = reader.read_i32(encryption).await?;

        // 1024 property is already a lot...
        if property_count < 0 || property_count > 1024 {
            return Err(Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Invalid property count: {}", property_count),
            ));
        }

        let mut properties = Vec::with_capacity(property_count as usize);

        // Add every property to the entity
        for _ in 0..property_count {
            // key and base value of the attribute
            let key = reader.read_string(encryption).await?;
            let base_value = reader.read_f64(encryption).await?;

            // Create a list of modifier (with a size)
            let modifier_count = reader.read_i16(encryption).await?;

            // 1024 modifier for an entity is biig
            if modifier_count < 0 || modifier_count > 1024 {
                return Err(Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!("Invalid modifier count: {}", modifier_count),
                ));
            }

            let mut modifiers: Vec<AttributeModifier> = Vec::with_capacity(modifier_count as usize);

            // Change the value with a modifier
            for _ in 0..modifier_count {
                // Create the UUID
                let uuid_most_sig = reader.read_i64(encryption).await?;
                let uuid_least_sig = reader.read_i64(encryption).await?;
                let uuid = Uuid::from_u64_pair(uuid_most_sig as u64, uuid_least_sig as u64);

                // Add the modifier (with the name, amount and operation)
                modifiers.push(AttributeModifier {
                    uuid,
                    name: "Unknown synced attribute modifier".to_string(),
                    amount: reader.read_f64(encryption).await?,
                    operation: reader.read_i8(encryption).await? as i32,
                });
            }

            properties.push(EntityProperty {
                key: AttributeKey::from_key(&key),
                base_value,
                modifiers,
            });
        }

        Ok(Self { entity, properties })
    }
}
