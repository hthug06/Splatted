use crate::network::connection::Encryption;
use std::io::Error;
use tokio::io::BufReader;
use tokio::net::tcp::OwnedReadHalf;

pub trait ServerPacket {
    async fn read(
        reader: &mut BufReader<OwnedReadHalf>,
        encryption: &mut Encryption,
    ) -> Result<Self, Error>
    where
        Self: Sized;
}

pub trait ClientPacket {
    fn write_to(&self, buffer: &mut Vec<u8>) -> Result<(), Error>;
}
