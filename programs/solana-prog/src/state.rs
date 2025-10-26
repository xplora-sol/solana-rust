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

/// User quest submission tracking
#[account]
pub struct QuestSubmission {
    /// User who submitted
    pub user: Pubkey,
    
    /// Quest location
    pub location: String,
    
    /// Quest index
    pub quest_index: u8,
    
    /// IPFS hash of submitted photo
    pub ipfs_hash: String,
    
    /// Optional user description
    pub description: String,
    
    /// Submission timestamp
    pub submitted_at: i64,
    
    /// Current status
    pub status: SubmissionStatus,
    
    /// Validator who approved/rejected
    pub validator: Option<Pubkey>,
    
    /// Validation timestamp
    pub validated_at: Option<i64>,
    
    /// Reward amount if approved
    pub reward_amount: u64,
    
    /// Number of attempts for this quest
    pub attempt_number: u8,
    
    /// PDA bump
    pub bump: u8,
}

impl QuestSubmission {
    pub const MAX_IPFS_HASH_LEN: usize = 64;
    pub const MAX_DESCRIPTION_LEN: usize = 200;
    
    pub fn space() -> usize {
        8 + // discriminator
        32 + // user pubkey
        4 + 64 + // location
        1 + // quest_index
        4 + Self::MAX_IPFS_HASH_LEN + // ipfs_hash
        4 + Self::MAX_DESCRIPTION_LEN + // description
        8 + // submitted_at
        1 + // status enum
        1 + 32 + // validator option
        1 + 8 + // validated_at option
        8 + // reward_amount
        1 + // attempt_number
        1 // bump
    }
}

/// Submission status enum
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub enum SubmissionStatus {
    Pending,
    Approved,
    Rejected,
}

/// User profile for progression tracking
#[account]
pub struct UserProfile {
    /// User wallet
    pub user: Pubkey,
    
    /// Optional username
    pub username: String,
    
    /// Profile creation timestamp
    pub created_at: i64,
    
    /// Last activity timestamp
    pub last_active: i64,
    
    /// Total quests completed
    pub quests_completed: u32,
    
    /// Total quests attempted
    pub quests_attempted: u32,
    
    /// Total XP earned
    pub experience_points: u64,
    
    /// Current level
    pub level: u16,
    
    /// Total tokens earned
    pub total_tokens_earned: u64,
    
    /// Unique locations visited
    pub unique_locations: u32,
    
    /// Current streak (consecutive days)
    pub current_streak: u16,
    
    /// Longest streak achieved
    pub longest_streak: u16,
    
    /// Last quest completion date (for streak tracking)
    pub last_quest_date: i64,
    
    /// Achievement bitmap (128 achievements)
    pub achievements: u128,
    
    /// Rank tier
    pub rank_tier: RankTier,
    
    /// PDA bump
    pub bump: u8,
}

impl UserProfile {
    pub const MAX_USERNAME_LEN: usize = 32;
    
    pub fn space() -> usize {
        8 + // discriminator
        32 + // user pubkey
        4 + Self::MAX_USERNAME_LEN + // username
        8 + // created_at
        8 + // last_active
        4 + // quests_completed
        4 + // quests_attempted
        8 + // experience_points
        2 + // level
        8 + // total_tokens_earned
        4 + // unique_locations
        2 + // current_streak
        2 + // longest_streak
        8 + // last_quest_date
        16 + // achievements u128
        1 + // rank_tier enum
        1 // bump
    }
    
    /// Calculate level from XP
    pub fn calculate_level(&self) -> u16 {
        (self.experience_points / 500) as u16
    }
    
    /// Calculate XP needed for next level
    pub fn xp_for_next_level(&self) -> u64 {
        ((self.level + 1) as u64) * 500
    }
}

/// User rank tiers
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub enum RankTier {
    Bronze,   // Levels 1-10
    Silver,   // Levels 11-25
    Gold,     // Levels 26-50
    Platinum, // Levels 51+
}

impl RankTier {
    pub fn from_level(level: u16) -> Self {
        match level {
            0..=10 => RankTier::Bronze,
            11..=25 => RankTier::Silver,
            26..=50 => RankTier::Gold,
            _ => RankTier::Platinum,
        }
    }
    
    pub fn token_multiplier(&self) -> f64 {
        match self {
            RankTier::Bronze => 1.0,
            RankTier::Silver => 1.2,
            RankTier::Gold => 1.5,
            RankTier::Platinum => 2.0,
        }
    }
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
    
    /// XP rewards
    pub const BASE_XP_REWARD: u64 = 100;
    pub const DIFFICULTY_MULTIPLIER_MEDIUM: f64 = 1.5;
    pub const DIFFICULTY_MULTIPLIER_HARD: f64 = 2.0;
    
    /// Token rewards (base amount)
    pub const BASE_TOKEN_REWARD: u64 = 100_000_000; // 0.1 tokens (assuming 9 decimals)
    
    /// Max attempts per quest
    pub const MAX_QUEST_ATTEMPTS: u8 = 3;
}
