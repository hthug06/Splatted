use std::io::Error;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProtocolVersion {
    V1_2,
    V1_3,
    V1_4,
    V1_5,
    V1_6,
}

impl ProtocolVersion {
    /// Create a ProtocolVersion from a real procotol version
    /// the real protocol version is stocked inside
    pub fn from_protocol_version(protocol_version: u32) -> Result<ProtocolVersion, Error> {
        match protocol_version {
            28..=29 => Ok(ProtocolVersion::V1_2),
            39 => Ok(ProtocolVersion::V1_3),
            47..=51 => Ok(ProtocolVersion::V1_4),
            60..=61 => Ok(ProtocolVersion::V1_5),
            72..=74 => Ok(ProtocolVersion::V1_6),
            78 => Ok(ProtocolVersion::V1_6),
            _ => Err(Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Unsupported protocol version: {}", protocol_version),
            )),
        }
    }
}
