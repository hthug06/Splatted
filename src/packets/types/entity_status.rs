/// The status of the entity.
/// 2 = Hurt | 3 = Dead
#[derive(Debug)]
pub enum EntityStatus {
    Hurt,
    Dead,
    Custom(u8),
}

impl EntityStatus {
    /// Get the status from the id
    pub fn from_id(id: u8) -> Self {
        match id {
            2 => EntityStatus::Hurt,
            3 => EntityStatus::Dead,
            other => EntityStatus::Custom(other),
        }
    }

    /// Get the id
    pub fn id(&self) -> u8 {
        match self {
            EntityStatus::Hurt => 2,
            EntityStatus::Dead => 3,
            EntityStatus::Custom(c) => *c,
        }
    }
}
