/// The status of the entity.
pub enum EntityStatus {
    Hurt,                 // 2
    Dead,                 // 3
    IronGolemAttack,      // 4
    TamingFailed,         // 6
    TamingSuccess,        // 7
    WolfShakingWater,     // 8
    EatingAccepted,       // 9
    SheepEatingGrass,     // 10
    IronGolemRose,        // 11
    VillagerMating,       // 12
    VillagerAngry,        // 13
    VillagerHappy,        // 14
    WitchMagicParticles,  // 15 (1.4+)
    ZombieVillagerCuring, // 16 (1.4+)
    FireworkExploding,    // 17 (1.5+)
    AnimalInLove,         // 18
    Custom(u8),
}

impl EntityStatus {
    /// Get the status from the id
    pub fn from_id(id: u8) -> Self {
        match id {
            2 => EntityStatus::Hurt,
            3 => EntityStatus::Dead,
            4 => EntityStatus::IronGolemAttack,
            6 => EntityStatus::TamingFailed,
            7 => EntityStatus::TamingSuccess,
            8 => EntityStatus::WolfShakingWater,
            9 => EntityStatus::EatingAccepted,
            10 => EntityStatus::SheepEatingGrass,
            11 => EntityStatus::IronGolemRose,
            12 => EntityStatus::VillagerMating,
            13 => EntityStatus::VillagerAngry,
            14 => EntityStatus::VillagerHappy,
            15 => EntityStatus::WitchMagicParticles,
            16 => EntityStatus::ZombieVillagerCuring,
            17 => EntityStatus::FireworkExploding,
            18 => EntityStatus::AnimalInLove,
            other => EntityStatus::Custom(other),
        }
    }

    /// Get the id
    pub fn id(&self) -> u8 {
        match self {
            EntityStatus::Hurt => 2,
            EntityStatus::Dead => 3,
            EntityStatus::IronGolemAttack => 4,
            EntityStatus::TamingFailed => 6,
            EntityStatus::TamingSuccess => 7,
            EntityStatus::WolfShakingWater => 8,
            EntityStatus::EatingAccepted => 9,
            EntityStatus::SheepEatingGrass => 10,
            EntityStatus::IronGolemRose => 11,
            EntityStatus::VillagerMating => 12,
            EntityStatus::VillagerAngry => 13,
            EntityStatus::VillagerHappy => 14,
            EntityStatus::WitchMagicParticles => 15,
            EntityStatus::ZombieVillagerCuring => 16,
            EntityStatus::FireworkExploding => 17,
            EntityStatus::AnimalInLove => 18,
            EntityStatus::Custom(c) => *c,
        }
    }
}
