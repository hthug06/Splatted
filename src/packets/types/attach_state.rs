#[derive(PartialEq, Eq)]
pub enum AttachState {
    Riding,      // 0
    Leashed,     // 1 : Attached with a lead (1.6+)
    Unknown(u8), // For plugin / modded servers
}

impl AttachState {
    pub fn from_id(id: u8) -> Self {
        match id {
            0 => AttachState::Riding,
            1 => AttachState::Leashed,
            other => AttachState::Unknown(other),
        }
    }
}
