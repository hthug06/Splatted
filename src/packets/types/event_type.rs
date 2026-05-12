/// The event type that can append in-game:
/// - Invalid bed
/// - rain start
/// - rain stop
/// - game mode changed
/// - custom (in case of mod, plugin, errors...)
pub enum EventType {
    InvalidBed,      // 0
    RainStarts,      // 1
    RainStops,       // 2
    GameModeChanged, // 3
    EnterCredits,    // 4
    DemoMessage,     // 5
    ArrowHit,        // 6 (1.6+)
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
            4 => EventType::EnterCredits,
            5 => EventType::DemoMessage,
            6 => EventType::ArrowHit,
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
            Self::EnterCredits => 4,
            Self::DemoMessage => 5,
            Self::ArrowHit => 6,
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
            Self::EnterCredits => "Enter credits",
            Self::DemoMessage => "Demo message",
            Self::ArrowHit => "Arrow hit",
            Self::Custom(_) => "custom",
        }
    }
}
