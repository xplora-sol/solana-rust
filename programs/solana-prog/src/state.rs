use anchor_lang::prelude::*;

/// Global registry that tracks all locations and manages authority
#[account]
pub struct QuestRegistry {
    /// Authority that can manage quests
    pub authority: Pubkey,
    
    /// Total number of locations with quests
    pub total_locations: u64,
    
    /// Version for future upgrades
    pub version: u8,
    
    /// Reserved space for future fields
    pub reserved: [u8; 7],
}

impl QuestRegistry {
    pub const LEN: usize = 8 + 32 + 8 + 1 + 7; // discriminator + fields
}

/// Per-location quest storage
#[account]
pub struct LocationQuests {
    /// Location identifier string
    pub location: String,
    
    /// Array of quests for this location
    pub quests: Vec<Quest>,
    
    /// Whether this account has been initialized
    pub initialized: bool,
    
    /// Creation timestamp
    pub created_at: i64,
    
    /// Last updated timestamp
    pub updated_at: i64,
    
    /// Reserved space for future fields
    pub reserved: [u8; 6],
}

impl LocationQuests {
    pub const MAX_QUESTS: usize = 10; // Reduced for memory constraints
    pub const MAX_LOCATION_LEN: usize = 64; // Reduced for reasonable sizing
    
    /// Calculate the maximum space needed for this account
    /// We allocate space for MAX_QUESTS to allow growing the Vec
    pub fn space() -> usize {
        8 + // discriminator
        4 + Self::MAX_LOCATION_LEN + // location string (4 bytes length + data)
        4 + (Self::MAX_QUESTS * Quest::max_size()) + // quests vec (4 bytes length + data)
        1 + // initialized bool
        8 + // created_at i64
        8 + // updated_at i64
        6 // reserved
    }
}

/// Individual quest data structure
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub struct Quest {
    /// Quest title
    pub title: String,
    
    /// Detailed quest description
    pub description: String,
    
    /// Type of quest
    pub quest_type: QuestType,
    
    /// Difficulty level
    pub difficulty: Difficulty,
    
    /// Time limit in hours
    pub time_to_live_hours: u16,
    
    /// What players need to verify
    pub verifiable_landmark: String,
    
    /// Name of the landmark
    pub landmark_name: String,
    
    /// GPS latitude
    pub latitude: f64,
    
    /// GPS longitude
    pub longitude: f64,
    
    /// Quest creation timestamp
    pub created_at: i64,
    
    /// Reserved space for future fields
    pub reserved: [u8; 4],
}

impl Quest {
    pub const MAX_TITLE_LEN: usize = 32;  // Reduced
    pub const MAX_DESCRIPTION_LEN: usize = 128;  // Reduced
    pub const MAX_LANDMARK_LEN: usize = 64;  // Reduced
    pub const MAX_LANDMARK_NAME_LEN: usize = 32;  // Reduced
    
    /// Calculate the maximum size of a Quest struct
    pub const fn max_size() -> usize {
        4 + Self::MAX_TITLE_LEN + // title
        4 + Self::MAX_DESCRIPTION_LEN + // description
        1 + // quest_type enum
        1 + // difficulty enum
        2 + // time_to_live_hours u16
        4 + Self::MAX_LANDMARK_LEN + // verifiable_landmark
        4 + Self::MAX_LANDMARK_NAME_LEN + // landmark_name
        8 + // latitude f64
        8 + // longitude f64
        8 + // created_at i64
        4 // reserved
    }
}

/// Types of quests available
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub enum QuestType {
    /// Discovery quests - finding specific items/places
    Discovery,
    
    /// Exploration quests - exploring areas
    Exploration,
    
    /// Challenge quests - completing tasks
    Challenge,
}

/// Difficulty levels for quests
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub enum Difficulty {
    /// Easy quests
    Easy,
    
    /// Medium difficulty quests
    Medium,
    
    /// Hard quests
    Hard,
}

/// Constants for validation
pub mod constants {
    /// Nepal's approximate latitude bounds
    pub const NEPAL_MIN_LATITUDE: f64 = 26.0;
    pub const NEPAL_MAX_LATITUDE: f64 = 31.0;
    
    /// Nepal's approximate longitude bounds
    pub const NEPAL_MIN_LONGITUDE: f64 = 80.0;
    pub const NEPAL_MAX_LONGITUDE: f64 = 89.0;
    
    /// Maximum quests per location
    pub const MAX_QUESTS_PER_LOCATION: usize = 10;
    
    /// Program version
    pub const PROGRAM_VERSION: u8 = 1;
}
