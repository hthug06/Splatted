use crate::network::connection::Encryption;
use crate::packets::io::MinecraftReadExt;
use crate::packets::packet_trait::ServerPacket;
use crate::packets::packet30_entity::EntityPacket;
use crate::packets::types::entity_animation::EntityAnimation;
use std::io::Error;
use tokio::io::BufReader;
use tokio::net::tcp::OwnedReadHalf;
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
            animation: EntityAnimation::from_id(reader.read_u8(encryption).await?),
        })
    }
}
