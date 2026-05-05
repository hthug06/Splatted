/// The game type of the player, in:
/// - Adventure
/// - Creative
/// - Survival
/// - Not Set
#[derive(PartialEq, Debug)]
pub enum GameType {
    NotSet,
    Survival,
    Creative,
    Adventure,
}

impl GameType {
    /// Get the game type from the id
    /// Used to parse it from the TCP stream
    pub fn from_id(id: i32) -> Option<Self> {
        match id {
            -1 => Some(Self::NotSet),
            0 => Some(Self::Survival),
            1 => Some(Self::Creative),
            2 => Some(Self::Adventure),
            _ => None,
        }
    }

    /// Get the id of the game type
    pub fn id(&self) -> i32 {
        match self {
            Self::NotSet => -1,
            Self::Survival => 0,
            Self::Creative => 1,
            Self::Adventure => 2,
        }
    }

    /// Get the name of the game type
    pub fn name(&self) -> &'static str {
        match self {
            Self::NotSet => "",
            Self::Survival => "survival",
            Self::Creative => "creative",
            Self::Adventure => "adventure",
        }
    }
}
