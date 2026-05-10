use crate::network::connection::Encryption;
use crate::packets::packet_trait::ServerPacket;
use crate::packets::types::instrument_type::InstrumentType;
use crate::packets::utils::{read_i16, read_i32, read_u8, read_u16};
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
            x: read_i32(reader, encryption).await?,
            y: read_i16(reader, encryption).await?,
            z: read_i32(reader, encryption).await?,
            instrument_type: InstrumentType::from_id(read_u8(reader, encryption).await?),
            pitch: read_u8(reader, encryption).await?,
            block_id: read_u16(reader, encryption).await?,
        })
    }
}
