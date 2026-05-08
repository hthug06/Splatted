/// All the world type that can be created:
/// - Default: The default world type, with all the biomes and structures.
/// - Flat: A flat world, with only grass and no structures.
/// - LargeBiomes: A world with larger biomes, but less structures.
/// - Default1_1: A world with the same generation as Minecraft 1.1.
/// - Unknown: An unknown world type, for plugins, mods, errors...
#[derive(Debug, PartialEq, Eq)]
pub enum WorldType {
    Default,
    Flat,
    LargeBiomes,
    Default1_1,
    Unknown(String), // Unknow world type for plugins, mod, or just glitch lol
}

impl WorldType {
    /// Parsing a world type from his name
    /// Used to parse it from the TCP stream
    pub fn parse(name: &str) -> Self {
        match name.to_lowercase().as_str() {
            "default" => WorldType::Default,
            "flat" => WorldType::Flat,
            "largebiomes" => WorldType::LargeBiomes,
            "default_1_1" => WorldType::Default1_1,
            _ => WorldType::Unknown(name.to_string()),
        }
    }

    /// Get the id of the world type
    pub fn id(&self) -> i32 {
        match self {
            WorldType::Default => 0,
            WorldType::Flat => 1,
            WorldType::LargeBiomes => 2,
            WorldType::Default1_1 => 8,
            WorldType::Unknown(_) => -1,
        }
    }

    /// Get the generator version of the world type
    pub fn generator_version(&self) -> i32 {
        match self {
            WorldType::Default => 1, // Supposons la version 1 pour le default
            WorldType::Default1_1 => 0,
            _ => 0, // Les autres n'ont pas forcément de version
        }
    }

    /// Check if the world type can be created
    pub fn can_be_created(&self) -> bool {
        !matches!(self, WorldType::Unknown(_))
    }

    /// Get the name of the world type
    pub fn name(&self) -> String {
        match self {
            WorldType::Default => "default".to_string(),
            WorldType::Flat => "flat".to_string(),
            WorldType::LargeBiomes => "largeBiomes".to_string(),
            WorldType::Default1_1 => "default_1_1".to_string(),
            WorldType::Unknown(name) => name.clone(),
        }
    }
}
