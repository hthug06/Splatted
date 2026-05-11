use crate::protocol_version::ProtocolVersion::Custom;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProtocolVersion {
    V1_2,
    V1_3,
    V1_4,
    V1_5,
    V1_6,
    Custom(u32),
}

impl ProtocolVersion {
    /// Create a ProtocolVersion from a real procotol version
    /// the real protocol version is stocked inside
    pub fn from_protocol_version(protocol_version: u32) -> ProtocolVersion {
        match protocol_version {
            28..=29 => ProtocolVersion::V1_2,
            39 => ProtocolVersion::V1_3,
            47..=51 => ProtocolVersion::V1_4,
            60..=61 => ProtocolVersion::V1_5,
            72..=74 => ProtocolVersion::V1_6,
            78 => ProtocolVersion::V1_6,
            _ => Custom(protocol_version),
        }
    }
}
