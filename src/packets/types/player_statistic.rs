#[derive(Debug, PartialEq, Eq)]
pub enum PlayerStatistic {
    // Base Stats
    LeaveGame,     // 1000
    PlayOneMinute, // 1001
    WalkOneCm,     // 1004
    SwimOneCm,     // 1005
    FallOneCm,     // 1006
    ClimbOneCm,    // 1007
    FlyOneCm,      // 1008
    DiveOneCm,     // 1009
    MinecartOneCm, // 1010
    BoatOneCm,     // 1011
    PigOneCm,      // 1012
    Jump,          // 1013
    Drop,          // 1014
    DamageDealt,   // 2020
    DamageTaken,   // 2021
    Deaths,        // 2022
    MobKills,      // 2023
    PlayerKills,   // 2024

    // These a 'weird' stats
    // These stats are calculated. You get a big number, and the stat is calculated with an offset.
    // You need to do : id of the stat - his offset
    // It's cool because Rust look like it's design for this
    CraftItem(i16), // Offset: 16777216
    MineBlock(i16), // Offset: 16842752
    UseItem(i16),   // Offset: 16908288
    BreakItem(i16), // Offset: 16973824

    // Mod or unknow stat
    Unknown(i32),
}

impl PlayerStatistic {
    pub fn from_id(id: i32) -> Self {
        match id {
            1000 => PlayerStatistic::LeaveGame,
            1001 => PlayerStatistic::PlayOneMinute,
            1004 => PlayerStatistic::WalkOneCm,
            1005 => PlayerStatistic::SwimOneCm,
            1006 => PlayerStatistic::FallOneCm,
            1007 => PlayerStatistic::ClimbOneCm,
            1008 => PlayerStatistic::FlyOneCm,
            1009 => PlayerStatistic::DiveOneCm,
            1010 => PlayerStatistic::MinecartOneCm,
            1011 => PlayerStatistic::BoatOneCm,
            1012 => PlayerStatistic::PigOneCm,
            1013 => PlayerStatistic::Jump,
            1014 => PlayerStatistic::Drop,
            2020 => PlayerStatistic::DamageDealt,
            2021 => PlayerStatistic::DamageTaken,
            2022 => PlayerStatistic::Deaths,
            2023 => PlayerStatistic::MobKills,
            2024 => PlayerStatistic::PlayerKills,

            // it's cool because here we can use the 'id if id >= offset' to know which stat it is.
            id if id >= 16973824 => PlayerStatistic::BreakItem((id - 16973824) as i16),
            id if id >= 16908288 => PlayerStatistic::UseItem((id - 16908288) as i16),
            id if id >= 16842752 => PlayerStatistic::MineBlock((id - 16842752) as i16),
            id if id >= 16777216 => PlayerStatistic::CraftItem((id - 16777216) as i16),

            other => PlayerStatistic::Unknown(other),
        }
    }
}
