#[derive(Debug, PartialEq, Eq)]
pub enum InstrumentType {
    Harp,        // 0
    BassDrum,    // 1
    SnareDrum,   // 2
    ClicksStick, // 3
    DoubleBass,  // 4
    // for mods, error...
    Custom(u8),
}

impl InstrumentType {
    pub fn from_id(id: u8) -> InstrumentType {
        match id {
            0 => InstrumentType::Harp,
            1 => InstrumentType::BassDrum,
            2 => InstrumentType::SnareDrum,
            3 => InstrumentType::ClicksStick,
            4 => InstrumentType::DoubleBass,
            other => InstrumentType::Custom(other),
        }
    }

    pub fn id(&self) -> u8 {
        match self {
            InstrumentType::Harp => 0,
            InstrumentType::BassDrum => 1,
            InstrumentType::SnareDrum => 2,
            InstrumentType::ClicksStick => 3,
            InstrumentType::DoubleBass => 4,
            InstrumentType::Custom(id) => *id,
        }
    }
}
