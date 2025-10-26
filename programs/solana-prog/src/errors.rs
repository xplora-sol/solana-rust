use anchor_lang::prelude::*;

#[error_code]
pub enum XploraError {
    #[msg("Empty quests array")]
    EmptyQuestsArray,
    
    #[msg("Too many quests (max 20)")]
    TooManyQuests,
    
    #[msg("Invalid quest index")]
    InvalidQuestIndex,
    
    #[msg("Unauthorized access")]
    Unauthorized,
    
    #[msg("Invalid quest title")]
    InvalidTitle,
    
    #[msg("Invalid quest description")]
    InvalidDescription,
    
    #[msg("Invalid verifiable landmark")]
    InvalidLandmark,
    
    #[msg("Invalid landmark name")]
    InvalidLandmarkName,
    
    #[msg("Invalid latitude (must be between 26.0 and 31.0)")]
    InvalidLatitude,
    
    #[msg("Invalid longitude (must be between 80.0 and 89.0)")]
    InvalidLongitude,
    
    #[msg("Arithmetic overflow")]
    Overflow,
    
    #[msg("Invalid location PDA")]
    InvalidLocationPDA,
    
    #[msg("Location already exists")]
    LocationAlreadyExists,
    
    #[msg("Location not found")]
    LocationNotFound,
    
    #[msg("Quest not found")]
    QuestNotFound,
    
    #[msg("Invalid quest type")]
    InvalidQuestType,
    
    #[msg("Invalid difficulty level")]
    InvalidDifficulty,
    
    #[msg("Account not initialized")]
    NotInitialized,
    
    #[msg("Account already initialized")]
    AlreadyInitialized,
}
