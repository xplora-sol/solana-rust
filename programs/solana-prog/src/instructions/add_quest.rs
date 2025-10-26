use anchor_lang::prelude::*;
use crate::state::*;
use crate::errors::XploraError;
use crate::utils::{validate_quest, get_current_timestamp};

/// Add a new quest to an existing location
pub fn add_quest_to_location(
    ctx: Context<crate::AddQuestToLocation>,
    quest: Quest,
) -> Result<()> {
    let location_quests = &mut ctx.accounts.location_quests;
    
    // Check if location is initialized
    require!(location_quests.initialized, XploraError::NotInitialized);
    
    // Check quest limit
    require!(
        location_quests.quests.len() < constants::MAX_QUESTS_PER_LOCATION,
        XploraError::TooManyQuests
    );
    
    // Validate quest data
    validate_quest(&quest)?;

    // Set creation timestamp
    let mut quest_with_timestamp = quest;
    quest_with_timestamp.created_at = get_current_timestamp();
    
    // Add quest to location
    location_quests.quests.push(quest_with_timestamp);
    location_quests.updated_at = get_current_timestamp();

    msg!("Added quest to location: {}", location_quests.location);
    msg!("Total quests: {}", location_quests.quests.len());

    Ok(())
}

