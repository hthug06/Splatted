use crate::network::connection::Encryption;
use crate::packets::packet_trait::ServerPacket;
use crate::packets::types::entity_metadata::EntityMetadata;
use crate::packets::utils::read_i32;
use std::io::Error;
use tokio::io::BufReader;
use tokio::net::tcp::OwnedReadHalf;

pub struct EntityMetadataPacket {
    pub id: i32,
    pub metadata: EntityMetadata,
}

impl ServerPacket for EntityMetadataPacket {
    async fn read(
        reader: &mut BufReader<OwnedReadHalf>,
        encryption: &mut Encryption,
    ) -> Result<Self, Error>
    where
        Self: Sized,
    {
        Ok(Self {
            id: read_i32(reader, encryption).await?,
            metadata: EntityMetadata::read(reader, encryption).await?,
        })
    }
}
