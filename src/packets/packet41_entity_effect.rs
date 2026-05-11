use crate::network::connection::Encryption;
use crate::packets::io::MinecraftReadExt;
use crate::packets::packet_trait::ServerPacket;
use crate::packets::packet30_entity::EntityPacket;
use crate::packets::types::potion_effect::PotionEffect;
use std::io::Error;
use tokio::io::BufReader;
use tokio::net::tcp::OwnedReadHalf;

/// Used when the player get a potion effect (don't get the packet when other player get an effect)
pub struct EntityEffectPacket {
    pub entity_id: EntityPacket,
    pub effect: PotionEffect,
    /// 0 = level 1, 1 = level 2 ....
    pub amplifier: i8,
    /// In ticks (20 ticks = 1 second)
    pub duration: i16,
}

impl ServerPacket for EntityEffectPacket {
    async fn read(
        reader: &mut BufReader<OwnedReadHalf>,
        encryption: &mut Encryption,
    ) -> Result<Self, Error> {
        Ok(Self {
            entity_id: EntityPacket::read(reader, encryption).await?,
            effect: PotionEffect::from_id(reader.read_u8(encryption).await?),
            amplifier: reader.read_i8(encryption).await?,
            duration: reader.read_i16(encryption).await?,
        })
    }
}
