use crate::network::connection::Encryption;
use crate::packets::io::{MinecraftReadExt, MinecraftWriteExt};
use crate::packets::packet_trait::{ClientPacket, ServerPacket};
use crate::protocol_version::ProtocolVersion;
use bytes::{BufMut, BytesMut};
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
        _protocol_version: ProtocolVersion,
    ) -> Result<Self, Error>
    where
        Self: Sized,
    {
        Ok(Self {
            x: reader.read_f64(encryption).await?,
            y: reader.read_f64(encryption).await?,
            stance: reader.read_f64(encryption).await?,
            z: reader.read_f64(encryption).await?,
            yaw: reader.read_f32(encryption).await?,
            pitch: reader.read_f32(encryption).await?,
            on_ground: reader.read_u8(encryption).await? != 0,
        })
    }
}

impl ClientPacket for PlayerLookMovePacket {
    fn write_to(
        &self,
        buffer: &mut BytesMut,
        _protocol_version: ProtocolVersion,
    ) -> Result<(), Error> {
        buffer.put_u8(13); // packet id

        // packet data
        buffer.put_f64(self.x);
        buffer.put_f64(self.y);
        buffer.put_f64(self.stance);
        buffer.put_f64(self.z);
        buffer.put_f32(self.yaw);
        buffer.put_f32(self.pitch);
        buffer.write_bool(self.on_ground);

        Ok(())
    }
}
