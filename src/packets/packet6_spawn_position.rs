use crate::network::connection::Encryption;
use crate::packets::io::MinecraftReadExt;
use crate::packets::packet_trait::ServerPacket;
use std::io::Error;
use tokio::io::BufReader;
use tokio::net::tcp::OwnedReadHalf;

pub struct SpawnPositionPacket {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl ServerPacket for SpawnPositionPacket {
    async fn read(
        reader: &mut BufReader<OwnedReadHalf>,
        encryption: &mut Encryption,
    ) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let x = reader.read_i32(encryption).await?;
        let y = reader.read_i32(encryption).await?;
        let z = reader.read_i32(encryption).await?;

        Ok(Self { x, y, z })
    }
}
