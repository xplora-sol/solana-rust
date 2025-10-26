use anchor_lang::prelude::*;
use crate::state::*;
use crate::errors::XploraError;

/// Validates quest data according to business rules
pub fn validate_quest(quest: &Quest) -> Result<()> {
    // Validate title
    require!(
        !quest.title.is_empty() && quest.title.len() <= Quest::MAX_TITLE_LEN,
        XploraError::InvalidTitle
    );
    
    // Validate description
    require!(
        !quest.description.is_empty() && quest.description.len() <= Quest::MAX_DESCRIPTION_LEN,
        XploraError::InvalidDescription
    );
    
    // Validate landmark
    require!(
        !quest.verifiable_landmark.is_empty() && quest.verifiable_landmark.len() <= Quest::MAX_LANDMARK_LEN,
        XploraError::InvalidLandmark
    );
    
    // Validate landmark name
    require!(
        !quest.landmark_name.is_empty() && quest.landmark_name.len() <= Quest::MAX_LANDMARK_NAME_LEN,
        XploraError::InvalidLandmarkName
    );
    
    // Validate coordinates (Nepal bounds)
    require!(
        quest.latitude >= constants::NEPAL_MIN_LATITUDE && 
        quest.latitude <= constants::NEPAL_MAX_LATITUDE,
        XploraError::InvalidLatitude
    );
    
    require!(
        quest.longitude >= constants::NEPAL_MIN_LONGITUDE && 
        quest.longitude <= constants::NEPAL_MAX_LONGITUDE,
        XploraError::InvalidLongitude
    );
    
    // Validate time to live
    require!(
        quest.time_to_live_hours > 0 && quest.time_to_live_hours <= 168, // Max 1 week
        XploraError::InvalidDifficulty
    );
    
    Ok(())
}

/// Validates location string
pub fn validate_location(location: &str) -> Result<()> {
    require!(
        !location.is_empty() && location.len() <= LocationQuests::MAX_LOCATION_LEN,
        XploraError::InvalidLandmarkName
    );
    
    Ok(())
}

/// Gets current timestamp
pub fn get_current_timestamp() -> i64 {
    Clock::get().unwrap().unix_timestamp
}

/// Derives the location quests PDA
pub fn derive_location_quests_pda(
    program_id: &Pubkey,
    location: &str,
) -> Result<(Pubkey, u8)> {
    Ok(Pubkey::find_program_address(
        &[b"location_quests", location.as_bytes()],
        program_id,
    ))
}

/// Derives the quest registry PDA
pub fn derive_quest_registry_pda(program_id: &Pubkey) -> Result<(Pubkey, u8)> {
    Ok(Pubkey::find_program_address(&[b"quest_registry"], program_id))
}
