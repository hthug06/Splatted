/// The event type that can append in-game:
/// - Invalid bed
/// - rain start
/// - rain stop
/// - game mode changed
/// - custom (in case of mod, plugin, errors...)
#[derive(Debug)]
pub enum EventType {
    InvalidBed,
    RainStarts,
    RainStops,
    GameModeChanged,
    Custom(i8),
}

impl EventType {
    /// Construct from the signed byte id used by the protocol (0, 1, 2, ...)
    pub fn from_id(id: i8) -> EventType {
        match id {
            0 => EventType::InvalidBed,
            1 => EventType::RainStarts,
            2 => EventType::RainStops,
            3 => EventType::GameModeChanged,
            _ => EventType::Custom(id),
        }
    }

    /// Return the numeric id corresponding to this variant (signed i8)
    pub fn id(&self) -> i8 {
        match *self {
            Self::InvalidBed => 0,
            Self::RainStarts => 1,
            Self::RainStops => 2,
            Self::GameModeChanged => 3,
            Self::Custom(v) => v,
        }
    }

    /// Human-friendly name (stable and useful for logging)
    pub fn name(&self) -> &'static str {
        match *self {
            Self::InvalidBed => "Invalid bed",
            Self::RainStarts => "Rain start",
            Self::RainStops => "Rain stop",
            Self::GameModeChanged => "Game mode changed",
            Self::Custom(_) => "custom",
        }
    }
}
