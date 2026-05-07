#[derive(Debug, PartialEq, Eq)]
pub enum EntityAnimation {
    SwingArm,            // 1 : Mine or attack
    Damage,              // 2 : Entity take damage (he becomes red :o)
    LeaveBed,            // 3 : Player wake up
    EatFood,             // 5 : Player eat or drink a potion
    CriticalEffect,      // 6 : Critical particles (brown stars)
    MagicCriticalEffect, // 7 : Enchantment critical particles (blue stars)

    // Mods, plugin,...
    Unknown(u8),
}

impl EntityAnimation {
    pub fn from_id(id: u8) -> Self {
        match id {
            1 => EntityAnimation::SwingArm,
            2 => EntityAnimation::Damage,
            3 => EntityAnimation::LeaveBed,
            5 => EntityAnimation::EatFood,
            6 => EntityAnimation::CriticalEffect,
            7 => EntityAnimation::MagicCriticalEffect,
            other => EntityAnimation::Unknown(other),
        }
    }
}
