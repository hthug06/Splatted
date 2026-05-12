use crate::network::connection::Encryption;
use crate::protocol_version::ProtocolVersion;
use bytes::BytesMut;
use std::io::Error;
use tokio::io::BufReader;
use tokio::net::tcp::OwnedReadHalf;

pub trait ServerPacket {
    async fn read(
        reader: &mut BufReader<OwnedReadHalf>,
        encryption: &mut Encryption,
        _protocol_version: ProtocolVersion,
    ) -> Result<Self, Error>
    where
        Self: Sized;
}

pub trait ClientPacket {
    fn write_to(
        &self,
        buffer: &mut BytesMut,
        _protocol_version: ProtocolVersion,
    ) -> Result<(), Error>;
}
