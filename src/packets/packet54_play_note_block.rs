use crate::network::connection::Encryption;
use crate::packets::io::MinecraftReadExt;
use crate::packets::packet_trait::ServerPacket;
use crate::protocol_version::ProtocolVersion;
use std::io::Error;
use tokio::io::BufReader;
use tokio::net::tcp::OwnedReadHalf;

/// Officially named BlockAction
/// Used for note blocks (`byte_1` = instrument type, `byte_2` = pitch),
/// pistons (`byte_1` = retracted or not, `byte_2` = direction),
/// and chests (`byte_1` = always 1, `byte_2` = whether the chest is open or closed).
pub struct PlayNoteBlockPacket {
    pub x: i32,
    pub y: i16,
    pub z: i32,
    /// Can be:
    /// - an Instrument type (note block)
    /// - if a piston is retracted or not
    /// - 1 for chest
    pub byte_1: u8,
    /// Can be:
    /// - pitch for note block
    /// - the direction of a piston
    /// - if the chest animation is opening or closing
    pub byte_2: u8,
    /// Implemented in 1.3
    /// Look if this is a note block, a piston or a chest
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
        let byte_1 = reader.read_u8(encryption).await?;
        let byte_2 = reader.read_u8(encryption).await?;

        // 1.2 don't check the block under
        let block_id = if protocol_version == ProtocolVersion::V1_2 {
            None
        } else {
            Some(reader.read_u16(encryption).await?)
        };

        Ok(Self {
            x,
            y,
            z,
            byte_1,
            byte_2,
            block_id,
        })
    }
}
