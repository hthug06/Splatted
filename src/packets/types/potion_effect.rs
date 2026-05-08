#[derive(Debug, PartialEq, Eq)]
/// All the potion effects in Minecraft 1.4.7
pub enum PotionEffect {
    Speed,
    Slowness,
    Haste,
    MiningFatigue,
    Strength,
    InstantHealth,
    InstantDamage,
    JumpBoost,
    Nausea,
    Regeneration,
    Resistance,
    FireResistance,
    WaterBreathing,
    Invisibility,
    Blindness,
    NightVision,
    Hunger,
    Weakness,
    Poison,
    Wither,

    // In case of mod once again
    Unknown(u8),
}

impl PotionEffect {
    /// Convertit l'ID reçu du réseau en PotionEffect
    pub fn from_id(id: u8) -> Self {
        match id {
            1 => PotionEffect::Speed,
            2 => PotionEffect::Slowness,
            3 => PotionEffect::Haste,
            4 => PotionEffect::MiningFatigue,
            5 => PotionEffect::Strength,
            6 => PotionEffect::InstantHealth,
            7 => PotionEffect::InstantDamage,
            8 => PotionEffect::JumpBoost,
            9 => PotionEffect::Nausea,
            10 => PotionEffect::Regeneration,
            11 => PotionEffect::Resistance,
            12 => PotionEffect::FireResistance,
            13 => PotionEffect::WaterBreathing,
            14 => PotionEffect::Invisibility,
            15 => PotionEffect::Blindness,
            16 => PotionEffect::NightVision,
            17 => PotionEffect::Hunger,
            18 => PotionEffect::Weakness,
            19 => PotionEffect::Poison,
            20 => PotionEffect::Wither,
            other => PotionEffect::Unknown(other),
        }
    }

    /// Get the id from the PotionEffect
    pub fn id(&self) -> u8 {
        match self {
            PotionEffect::Speed => 1,
            PotionEffect::Slowness => 2,
            PotionEffect::Haste => 3,
            PotionEffect::MiningFatigue => 4,
            PotionEffect::Strength => 5,
            PotionEffect::InstantHealth => 6,
            PotionEffect::InstantDamage => 7,
            PotionEffect::JumpBoost => 8,
            PotionEffect::Nausea => 9,
            PotionEffect::Regeneration => 10,
            PotionEffect::Resistance => 11,
            PotionEffect::FireResistance => 12,
            PotionEffect::WaterBreathing => 13,
            PotionEffect::Invisibility => 14,
            PotionEffect::Blindness => 15,
            PotionEffect::NightVision => 16,
            PotionEffect::Hunger => 17,
            PotionEffect::Weakness => 18,
            PotionEffect::Poison => 19,
            PotionEffect::Wither => 20,

            PotionEffect::Unknown(id) => *id,
        }
    }
}
