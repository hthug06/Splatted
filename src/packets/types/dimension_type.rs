use std::fmt;

#[derive(Debug, PartialEq, Eq)]
/// The dimension type of the player, in:
/// - Nether
/// - Overworld
/// - End
/// - Custom dimension with an id (u8)
pub enum DimensionType {
    Nether,
    Overworld,
    End,
    Custom(i8),
}

impl DimensionType {
    /// Construct from the signed byte id used by the protocol (-1, 0, 1, ...)
    pub fn from_id(id: i8) -> Self {
        match id {
            -1 => Self::Nether,
            0 => Self::Overworld,
            1 => Self::End,
            other => Self::Custom(other),
        }
    }

    /// Return the numeric id corresponding to this variant (signed i8)
    pub fn id(&self) -> i8 {
        match *self {
            Self::Nether => -1,
            Self::Overworld => 0,
            Self::End => 1,
            Self::Custom(v) => v,
        }
    }

    /// Human-friendly name (stable and useful for logging)
    pub fn name(&self) -> &'static str {
        match *self {
            Self::Nether => "nether",
            Self::Overworld => "overworld",
            Self::End => "end",
            Self::Custom(_) => "custom",
        }
    }
}

impl fmt::Display for DimensionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Nether => write!(f, "Nether (id={})", self.id()),
            Self::Overworld => write!(f, "Overworld (id={})", self.id()),
            Self::End => write!(f, "End (id={})", self.id()),
            Self::Custom(v) => write!(f, "Custom({})", v),
        }
    }
}
