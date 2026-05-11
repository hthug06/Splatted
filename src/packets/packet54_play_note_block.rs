use crate::network::connection::Encryption;
use crate::packets::io::MinecraftReadExt;
use crate::packets::packet_trait::ServerPacket;
use crate::packets::types::instrument_type::InstrumentType;
use std::io::Error;
use tokio::io::BufReader;
use tokio::net::tcp::OwnedReadHalf;
pub struct PlayNoteBlockPacket {
    pub x: i32,
    pub y: i16,
    pub z: i32,
    pub instrument_type: InstrumentType,
    pub pitch: u8,
    pub block_id: u16,
}

impl ServerPacket for PlayNoteBlockPacket {
    async fn read(
        reader: &mut BufReader<OwnedReadHalf>,
        encryption: &mut Encryption,
    ) -> Result<Self, Error>
    where
        Self: Sized,
    {
        Ok(Self {
            x: reader.read_i32(encryption).await?,
            y: reader.read_i16(encryption).await?,
            z: reader.read_i32(encryption).await?,
            instrument_type: InstrumentType::from_id(reader.read_u8(encryption).await?),
            pitch: reader.read_u8(encryption).await?,
            block_id: reader.read_u16(encryption).await?,
        })
    }
}
