use std::io::{Cursor, Error};

mod packet253_server_auth_data;
pub mod packet254_server_ping;
pub mod packet255_kick_disconnect;
pub mod packet2_client_protocol;

pub trait ServerPacket {
    fn read(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error>
    where
        Self: Sized;
}

pub trait ClientPacket {
    fn write_to(&self, buffer: &mut Vec<u8>) -> Result<(), Error>;
}
