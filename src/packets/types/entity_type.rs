/// All the entity type of minecraft 1.4.7
/// For now, it's hardcoded because the project focus only 1.4.7.
/// Later, if we want multiversion, we might need to generate code with all the dataData
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntityType {
    // Object and Projectiles
    Item,                  // 1
    XPOrb,                 // 2
    Painting,              // 9
    Arrow,                 // 10
    Snowball,              // 11
    Fireball,              // 12
    SmallFireball,         // 13
    ThrownEnderpearl,      // 14
    EyeOfEnderSignal,      // 15
    ThrownPotion,          // 16
    ThrownExpBottle,       // 17
    ItemFrame,             // 18
    WitherSkull,           // 19
    PrimedTnt,             // 20
    FallingSand,           // 21
    FireworksRocketEntity, // 22

    // Vehicules
    Minecart, // 40
    Boat,     // 41

    // Mobs (generic)
    Mob,     // 48
    Monster, // 49

    // Hostiles Mobs
    Creeper,     // 50
    Skeleton,    // 51
    Spider,      // 52
    Giant,       // 53
    Zombie,      // 54
    Slime,       // 55
    Ghast,       // 56
    PigZombie,   // 57
    Enderman,    // 58
    CaveSpider,  // 59
    Silverfish,  // 60
    Blaze,       // 61
    LavaSlime,   // 62 (Magma Cube wtf?)
    EnderDragon, // 63
    WitherBoss,  // 64
    Bat,         // 65
    Witch,       // 66

    // Passives Mobs (animals)
    Pig,           // 90
    Sheep,         // 91
    Cow,           // 92
    Chicken,       // 93
    Squid,         // 94
    Wolf,          // 95
    MushroomCow,   // 96 (Mooshroom)
    SnowMan,       // 97
    Ocelot,        // 98
    VillagerGolem, // 99 (Iron Golem)
    Villager,      // 120

    // Other
    EnderCrystal, // 200

    // In case the mobs id is unknow (once again, mods)
    Unknown(u8),
}

impl EntityType {
    /// Convert ID to an entity type
    pub fn from_id(id: u8) -> Self {
        match id {
            1 => EntityType::Item,
            2 => EntityType::XPOrb,
            9 => EntityType::Painting,
            10 => EntityType::Arrow,
            11 => EntityType::Snowball,
            12 => EntityType::Fireball,
            13 => EntityType::SmallFireball,
            14 => EntityType::ThrownEnderpearl,
            15 => EntityType::EyeOfEnderSignal,
            16 => EntityType::ThrownPotion,
            17 => EntityType::ThrownExpBottle,
            18 => EntityType::ItemFrame,
            19 => EntityType::WitherSkull,
            20 => EntityType::PrimedTnt,
            21 => EntityType::FallingSand,
            22 => EntityType::FireworksRocketEntity,

            40 => EntityType::Minecart,
            41 => EntityType::Boat,

            48 => EntityType::Mob,
            49 => EntityType::Monster,

            50 => EntityType::Creeper,
            51 => EntityType::Skeleton,
            52 => EntityType::Spider,
            53 => EntityType::Giant,
            54 => EntityType::Zombie,
            55 => EntityType::Slime,
            56 => EntityType::Ghast,
            57 => EntityType::PigZombie,
            58 => EntityType::Enderman,
            59 => EntityType::CaveSpider,
            60 => EntityType::Silverfish,
            61 => EntityType::Blaze,
            62 => EntityType::LavaSlime,
            63 => EntityType::EnderDragon,
            64 => EntityType::WitherBoss,
            65 => EntityType::Bat,
            66 => EntityType::Witch,

            90 => EntityType::Pig,
            91 => EntityType::Sheep,
            92 => EntityType::Cow,
            93 => EntityType::Chicken,
            94 => EntityType::Squid,
            95 => EntityType::Wolf,
            96 => EntityType::MushroomCow,
            97 => EntityType::SnowMan,
            98 => EntityType::Ocelot,
            99 => EntityType::VillagerGolem,
            120 => EntityType::Villager,

            200 => EntityType::EnderCrystal,

            other => EntityType::Unknown(other),
        }
    }
}
