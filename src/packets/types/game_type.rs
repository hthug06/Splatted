/// The game type of the player, in:
/// - Adventure
/// - Creative
/// - Survival
/// - Not Set
#[derive(PartialEq, Eq)]
pub enum GameType {
    NotSet,
    Survival,
    Creative,
    Adventure,
}

impl GameType {
    /// Get the game type from the id
    /// Used to parse it from the TCP stream
    /// Default is survival
    pub fn from_id(id: i8) -> Self {
        match id {
            -1 => Self::NotSet,
            1 => Self::Creative,
            2 => Self::Adventure,
            _ => Self::Survival,
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
