use crate::network::connection::Encryption;
use crate::packets::packet_trait::ServerPacket;
use crate::packets::utils::{read_f32, read_f64, read_i8, read_i32};
use std::io::{Error, ErrorKind};
use tokio::io::BufReader;
use tokio::net::tcp::OwnedReadHalf;
const MAX_DESTROYED_BLOCKS: i32 = 100_000;

#[derive(Debug)]
pub struct ExplosionPacket {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub radius: f32,

    /// The coordinates of every destroyed blocks
    pub destroyed_blocks: Vec<(i32, i32, i32)>,

    // The velocity the player will get
    pub velocity_x: f32,
    pub velocity_y: f32,
    pub velocity_z: f32,
}

impl ServerPacket for ExplosionPacket {
    async fn read(
        reader: &mut BufReader<OwnedReadHalf>,
        encryption: &mut Encryption,
    ) -> Result<Self, Error> {
        let x = read_f64(reader, encryption).await?;
        let y = read_f64(reader, encryption).await?;
        let z = read_f64(reader, encryption).await?;
        let radius = read_f32(reader, encryption).await?;

        let destroyed_block_count = read_i32(reader, encryption).await?;

        // Check if this explosion is too big
        if !(0..=MAX_DESTROYED_BLOCKS).contains(&destroyed_block_count) {
            return Err(Error::new(
                ErrorKind::InvalidData,
                format!(
                    "Explosion destroyed_block_count is invalid or too big: {}",
                    destroyed_block_count
                ),
            ));
        }
        // The coordinate of where the explosion start
        let base_x = x as i32;
        let base_y = y as i32;
        let base_z = z as i32;

        let mut destroyed_blocks = Vec::with_capacity(destroyed_block_count as usize);

        for _ in 0..destroyed_block_count {
            let offset_x = read_i8(reader, encryption).await? as i32;
            let offset_y = read_i8(reader, encryption).await? as i32;
            let offset_z = read_i8(reader, encryption).await? as i32;

            destroyed_blocks.push((base_x + offset_x, base_y + offset_y, base_z + offset_z));
        }

        let velocity_x = read_f32(reader, encryption).await?;
        let velocity_y = read_f32(reader, encryption).await?;
        let velocity_z = read_f32(reader, encryption).await?;

        Ok(Self {
            x,
            y,
            z,
            radius,
            destroyed_blocks,
            velocity_x,
            velocity_y,
            velocity_z,
        })
    }
}
