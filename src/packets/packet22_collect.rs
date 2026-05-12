use crate::network::connection::Encryption;
use crate::packets::packet_trait::ServerPacket;
use crate::packets::packet30_entity::EntityPacket;
use crate::protocol_version::ProtocolVersion;
use std::io::Error;
use tokio::io::BufReader;
use tokio::net::tcp::OwnedReadHalf;

pub struct CollectPacket {
    /// The collected entity aka the item you get in your inventory (Item, xp orb, arrow...)
    pub collected_entity: EntityPacket,
    /// The entity that collected the item (Player, mob...)
    pub collector_entity: EntityPacket,
}

impl ServerPacket for CollectPacket {
    async fn read(
        reader: &mut BufReader<OwnedReadHalf>,
        encryption: &mut Encryption,
        protocol_version: ProtocolVersion,
    ) -> Result<Self, Error>
    where
        Self: Sized,
    {
        Ok(Self {
            collected_entity: EntityPacket::read(reader, encryption, protocol_version).await?,
            collector_entity: EntityPacket::read(reader, encryption, protocol_version).await?,
        })
    }
}
