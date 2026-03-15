use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct WrongPacketError {
    pub attended: u8,
    pub received: u8,
}

impl fmt::Display for WrongPacketError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "invalid packet received, should receive {}, but received {}",
            self.attended, self.received
        )
    }
}

impl Error for WrongPacketError {}
