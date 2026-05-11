use crate::network::connection::Encryption;
use crate::packets::io::MinecraftReadExt;
use crate::packets::packet_trait::ServerPacket;
use crate::packets::types::instrument_type::InstrumentType;
use crate::protocol_version::ProtocolVersion;
use std::io::Error;
use tokio::io::BufReader;
use tokio::net::tcp::OwnedReadHalf;

pub struct PlayNoteBlockPacket {
    pub x: i32,
    pub y: i16,
    pub z: i32,
    pub instrument_type: InstrumentType,
    pub pitch: u8,
    /// Only for 1.4
    /// I think Minecraft don't look the block under to change the sound
    pub block_id: Option<u16>,
}

impl ServerPacket for PlayNoteBlockPacket {
    async fn read(
        reader: &mut BufReader<OwnedReadHalf>,
        encryption: &mut Encryption,
        protocol_version: ProtocolVersion,
    ) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let x = reader.read_i32(encryption).await?;
        let y = reader.read_i16(encryption).await?;
        let z = reader.read_i32(encryption).await?;
        let instrument_type = InstrumentType::from_id(reader.read_u8(encryption).await?);
        let pitch = reader.read_u8(encryption).await?;

        // 1.4
        let block_id = if protocol_version == ProtocolVersion::V1_4 {
            Some(reader.read_u16(encryption).await?)
        }
        //1.2
        else {
            None
        };

        Ok(Self {
            x,
            y,
            z,
            instrument_type,
            pitch,
            block_id,
        })
    }
}
