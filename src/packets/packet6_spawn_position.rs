use crate::network::connection::Encryption;
use crate::packets::packet_trait::ServerPacket;
use crate::packets::utils::read_i32;
use std::io::Error;
use tokio::io::BufReader;
use tokio::net::tcp::OwnedReadHalf;

#[derive(Debug)]
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
        let x = read_i32(reader, encryption).await?;
        let y = read_i32(reader, encryption).await?;
        let z = read_i32(reader, encryption).await?;

        Ok(Self { x, y, z })
    }
}
