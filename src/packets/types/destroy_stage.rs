/// The Destroy stage of a block
/// The block can be cracked (value 0 - 9) or reset (value 255)
///
#[derive(Debug, PartialEq, Eq)]
pub enum DestroyStage {
    /// The block is being mined (value 0-9)
    Cracking(u8),
    /// Player stop mined the block
    Reset,
    /// In case an error from the server (or the parsing...)
    Unknown(u8),
}

impl DestroyStage {
    /// Get the destroy stage from the id
    pub fn from_id(id: u8) -> Self {
        match id {
            0..=9 => DestroyStage::Cracking(id),
            255 => DestroyStage::Reset,
            other => DestroyStage::Unknown(other),
        }
    }
}
