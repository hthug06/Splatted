use crate::network::connection::Encryption;
use crate::packets::io::MinecraftReadExt;
use crate::packets::packet_trait::ServerPacket;
use crate::packets::packet30_entity::EntityPacket;
use std::io::Error;
use tokio::io::BufReader;
use tokio::net::tcp::OwnedReadHalf;

pub struct RelEntityMovePacket {
    pub entity: EntityPacket,
    pub x: i8,
    pub y: i8,
    pub z: i8,
}

impl ServerPacket for RelEntityMovePacket {
    async fn read(
        reader: &mut BufReader<OwnedReadHalf>,
        encryption: &mut Encryption,
    ) -> Result<Self, Error>
    where
        Self: Sized,
    {
        Ok(Self {
            entity: EntityPacket::read(reader, encryption).await?,
            x: reader.read_i8(encryption).await?,
            y: reader.read_i8(encryption).await?,
            z: reader.read_i8(encryption).await?,
        })
    }
}
