/// Represents an Entity attribute key
/// Implemented in 1.6 for packet 44: Update Attribute
#[derive(PartialEq, Eq)]
pub enum AttributeKey {
    MaxHealth,                 // "generic.maxHealth"
    FollowRange,               // "generic.followRange"
    KnockbackResistance,       // "generic.knockbackResistance"
    MovementSpeed,             // "generic.movementSpeed"
    AttackDamage,              // "generic.attackDamage"
    HorseJumpStrength,         // "horse.jumpStrength"
    ZombieSpawnReinforcements, // "zombie.spawnReinforcements"

    // For mod or plugins
    Custom(String),
}

impl AttributeKey {
    /// Convert a string attribute key to an Attribute Key
    pub fn from_key(key: &str) -> Self {
        match key {
            "generic.maxHealth" => AttributeKey::MaxHealth,
            "generic.followRange" => AttributeKey::FollowRange,
            "generic.knockbackResistance" => AttributeKey::KnockbackResistance,
            "generic.movementSpeed" => AttributeKey::MovementSpeed,
            "generic.attackDamage" => AttributeKey::AttackDamage,
            "horse.jumpStrength" => AttributeKey::HorseJumpStrength,
            "zombie.spawnReinforcements" => AttributeKey::ZombieSpawnReinforcements,
            _ => AttributeKey::Custom(key.to_string()),
        }
    }

    /// Get the Key from this enum
    pub fn as_str(&self) -> &str {
        match self {
            AttributeKey::MaxHealth => "generic.maxHealth",
            AttributeKey::FollowRange => "generic.followRange",
            AttributeKey::KnockbackResistance => "generic.knockbackResistance",
            AttributeKey::MovementSpeed => "generic.movementSpeed",
            AttributeKey::AttackDamage => "generic.attackDamage",
            AttributeKey::HorseJumpStrength => "horse.jumpStrength",
            AttributeKey::ZombieSpawnReinforcements => "zombie.spawnReinforcements",
            AttributeKey::Custom(s) => s.as_str(),
        }
    }
}
