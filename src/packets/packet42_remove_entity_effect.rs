use crate::network::connection::Encryption;
use crate::packets::packet_trait::ServerPacket;
use crate::packets::packet30_entity::EntityPacket;
use crate::packets::types::potion_effect::PotionEffect;
use crate::packets::utils::read_u8;
use std::io::Error;
use tokio::io::BufReader;
use tokio::net::tcp::OwnedReadHalf;

#[derive(Debug)]
pub struct RemoveEntityEffectPacket {
    pub entity: EntityPacket,
    pub effect: PotionEffect,
}

impl ServerPacket for RemoveEntityEffectPacket {
    async fn read(
        reader: &mut BufReader<OwnedReadHalf>,
        encryption: &mut Encryption,
    ) -> Result<Self, Error>
    where
        Self: Sized,
    {
        Ok(Self {
            entity: EntityPacket::read(reader, encryption).await?,
            effect: PotionEffect::from_id(read_u8(reader, encryption).await?),
        })
    }
}
