use std::error::Error;
use std::fmt;

type Result<T> = std::result::Result<T, WrongPacketError>;

#[derive(Debug)]
pub struct WrongPacketError {
    pub right_packet: u8,
    pub wrong_packet: u8,
}

impl fmt::Display for WrongPacketError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid packet received, should receive {}, but received {}", self.right_packet, self.wrong_packet)
    }
}

impl Error for WrongPacketError {}
