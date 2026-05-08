use crate::network::connection::Encryption;
use crate::packets::packet_trait::ServerPacket;
use crate::packets::packet30_entity::EntityPacket;
use crate::packets::types::entity_animation::EntityAnimation;
use crate::packets::utils::read_u8;
use std::io::Error;
use tokio::io::BufReader;
use tokio::net::tcp::OwnedReadHalf;
#[derive(Debug)]
pub struct AnimationPacket {
    entity: EntityPacket,
    animation: EntityAnimation,
}

impl ServerPacket for AnimationPacket {
    async fn read(
        reader: &mut BufReader<OwnedReadHalf>,
        encryption: &mut Encryption,
    ) -> Result<Self, Error>
    where
        Self: Sized,
    {
        Ok(Self {
            entity: EntityPacket::read(reader, encryption).await?,
            animation: EntityAnimation::from_id(read_u8(reader, encryption).await?),
        })
    }
}
