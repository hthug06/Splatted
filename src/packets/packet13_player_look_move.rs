use crate::network::connection::Encryption;
use crate::packets::packet_trait::{ClientPacket, ServerPacket};
use crate::packets::utils::{read_f32, read_f64, read_u8, write_bool, write_f32, write_f64};
use std::io::Error;
use tokio::io::BufReader;
use tokio::net::tcp::OwnedReadHalf;

pub struct PlayerLookMovePacket {
    pub x: f64,
    pub y: f64,
    pub stance: f64,
    pub z: f64,
    pub yaw: f32,
    pub pitch: f32,
    pub on_ground: bool, // flatten the FlyingPacket
}

impl ServerPacket for PlayerLookMovePacket {
    async fn read(
        reader: &mut BufReader<OwnedReadHalf>,
        encryption: &mut Encryption,
    ) -> Result<Self, Error>
    where
        Self: Sized,
    {
        // Reverse the y and stance when reading (idk why Notch did this)
        Ok(Self {
            x: read_f64(reader, encryption).await?,
            stance: read_f64(reader, encryption).await?,
            y: read_f64(reader, encryption).await?,
            z: read_f64(reader, encryption).await?,
            yaw: read_f32(reader, encryption).await?,
            pitch: read_f32(reader, encryption).await?,
            on_ground: read_u8(reader, encryption).await? != 0,
        })
    }
}

impl ClientPacket for PlayerLookMovePacket {
    fn write_to(&self, buffer: &mut Vec<u8>) -> Result<(), Error> {
        buffer.push(13); // packet id

        // packet data
        write_f64(buffer, self.x);
        write_f64(buffer, self.y);
        write_f64(buffer, self.stance);
        write_f64(buffer, self.z);
        write_f32(buffer, self.yaw);
        write_f32(buffer, self.pitch);
        write_bool(buffer, self.on_ground);

        Ok(())
    }
}
