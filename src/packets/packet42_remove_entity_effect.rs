use crate::network::connection::Encryption;
use crate::packets::io::MinecraftReadExt;
use crate::packets::packet_trait::ServerPacket;
use crate::packets::packet30_entity::EntityPacket;
use crate::packets::types::potion_effect::PotionEffect;
use std::io::Error;
use tokio::io::BufReader;
use tokio::net::tcp::OwnedReadHalf;

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
            effect: PotionEffect::from_id(reader.read_u8(encryption).await?),
        })
    }
}
