//! Every song/effect in minecraft 1.4.7
//! In the source code, they only use id, there no such enum
//! But i think it's better to have one in rust
#[derive(PartialEq, Eq)]
pub enum SoundEffect {
    // Minors song (1000+)
    DispenserDispense,
    DispenserFail,
    DispenserShoot,
    DoorToggle,  // Door, Trapdoor, Fence Gate
    Fizz,        // Lava in water or redstone torch that burns
    PlayRecord,  // Jukebox
    GhastCharge, // Ghast sound before he fires a fireball
    GhastShoot,  // When the ghast fire
    GhastFireballExplode,
    ZombieAttackWoodDoor,
    ZombieBreakWoodDoor,
    ZombieAttackIronDoor,
    WitherShoot,
    BatTakeoff,
    ZombieInfectVillager,
    ZombieVillagerCured,
    EnderDragonDeath,
    AnvilBreak,
    AnvilUse,
    AnvilLand,

    // Particles and blocks (2000+)
    SpawnSmoke,       // 9 smoke particles (ex: dispenser)
    BlockBreak,       // Broken block particles + Sound (Block ID is in auxData)
    SplashPotion,     // Potion explosion (Color is in auxData)
    EyeOfEnderBreak,  // Ender eye particles breaking
    MobSpawnParticle, // Mob spawning from a spawner
    BonemealParticle, // Bone meal used on a plant

    // Once again, for mods, plugin, etc...
    Unknown(i32),
}

impl SoundEffect {
    /// Convertit the id into a SoundEffect
    pub fn from_id(id: i32) -> Self {
        match id {
            1000 => SoundEffect::DispenserDispense,
            1001 => SoundEffect::DispenserFail,
            1002 => SoundEffect::DispenserShoot,
            1003 => SoundEffect::DoorToggle,
            1004 => SoundEffect::Fizz,
            1005 => SoundEffect::PlayRecord,
            1007 => SoundEffect::GhastCharge,
            1008 => SoundEffect::GhastShoot,
            1009 => SoundEffect::GhastFireballExplode,
            1010 => SoundEffect::ZombieAttackWoodDoor,
            1011 => SoundEffect::ZombieBreakWoodDoor,
            1012 => SoundEffect::ZombieAttackIronDoor,
            1014 => SoundEffect::WitherShoot,
            1015 => SoundEffect::BatTakeoff,
            1016 => SoundEffect::ZombieInfectVillager,
            1017 => SoundEffect::ZombieVillagerCured,
            1018 => SoundEffect::EnderDragonDeath,
            1020 => SoundEffect::AnvilBreak,
            1021 => SoundEffect::AnvilUse,
            1022 => SoundEffect::AnvilLand,

            2000 => SoundEffect::SpawnSmoke,
            2001 => SoundEffect::BlockBreak,
            2002 => SoundEffect::SplashPotion,
            2003 => SoundEffect::EyeOfEnderBreak,
            2004 => SoundEffect::MobSpawnParticle,
            2005 => SoundEffect::BonemealParticle,

            other => SoundEffect::Unknown(other),
        }
    }

    /// Convert the sound effect into an id
    pub fn id(&self) -> i32 {
        match self {
            SoundEffect::DispenserDispense => 1000,
            SoundEffect::DispenserFail => 1001,
            SoundEffect::DispenserShoot => 1002,
            SoundEffect::DoorToggle => 1003,
            SoundEffect::Fizz => 1004,
            SoundEffect::PlayRecord => 1005,
            SoundEffect::GhastCharge => 1007,
            SoundEffect::GhastShoot => 1008,
            SoundEffect::GhastFireballExplode => 1009,
            SoundEffect::ZombieAttackWoodDoor => 1010,
            SoundEffect::ZombieBreakWoodDoor => 1011,
            SoundEffect::ZombieAttackIronDoor => 1012,
            SoundEffect::WitherShoot => 1014,
            SoundEffect::BatTakeoff => 1015,
            SoundEffect::ZombieInfectVillager => 1016,
            SoundEffect::ZombieVillagerCured => 1017,
            SoundEffect::EnderDragonDeath => 1018,
            SoundEffect::AnvilBreak => 1020,
            SoundEffect::AnvilUse => 1021,
            SoundEffect::AnvilLand => 1022,

            SoundEffect::SpawnSmoke => 2000,
            SoundEffect::BlockBreak => 2001,
            SoundEffect::SplashPotion => 2002,
            SoundEffect::EyeOfEnderBreak => 2003,
            SoundEffect::MobSpawnParticle => 2004,
            SoundEffect::BonemealParticle => 2005,

            // Si c'est un unknown, on renvoie simplement la valeur qu'il contient !
            SoundEffect::Unknown(id) => *id,
        }
    }
}
