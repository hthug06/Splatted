/// Used in the NetClientHandler line 1188 - 1207
/// This is more clear how the Tile entity will be used
#[derive(PartialEq, Eq)]
pub enum TileEntityAction {
    MobSpawner,   // Action = 1
    CommandBlock, // Action = 2
    Beacon,       // Action = 3
    Skull,        // Action = 4
    Unknown(u8),  // for mods
}

impl TileEntityAction {
    /// Convert from an u8
    pub fn from_id(value: u8) -> Self {
        match value {
            1 => TileEntityAction::MobSpawner,
            2 => TileEntityAction::CommandBlock,
            3 => TileEntityAction::Beacon,
            4 => TileEntityAction::Skull,
            other => TileEntityAction::Unknown(other),
        }
    }
}
