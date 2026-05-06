use crate::network::connection::Encryption;
use crate::packets::packet_trait::ServerPacket;
use crate::packets::packet10_flying::FlyingPacket;
use crate::packets::utils::{read_f32, read_f64};
use std::io::Error;
use tokio::io::BufReader;
use tokio::net::tcp::OwnedReadHalf;

#[derive(Debug)]
pub struct PlayerLookMovePacket {
    pub x: f64,
    pub y: f64,
    pub stance: f64,
    pub z: f64,
    pub yaw: f32,
    pub pitch: f32,
    pub flying: FlyingPacket,
}

impl ServerPacket for PlayerLookMovePacket {
    async fn read(
        reader: &mut BufReader<OwnedReadHalf>,
        encryption: &mut Encryption,
    ) -> Result<Self, Error>
    where
        Self: Sized,
    {
        Ok(Self {
            x: read_f64(reader, encryption).await?,
            y: read_f64(reader, encryption).await?,
            stance: read_f64(reader, encryption).await?,
            z: read_f64(reader, encryption).await?,
            yaw: read_f32(reader, encryption).await?,
            pitch: read_f32(reader, encryption).await?,
            flying: FlyingPacket::read(reader, encryption).await?,
        })
    }
}
