use anchor_lang::prelude::*;
use crate::state::*;
use crate::errors::XploraError;
use crate::utils::{validate_location, validate_quest, get_current_timestamp, derive_location_quests_pda};

/// Create quests for a new location
pub fn create_location_quests(
    ctx: Context<crate::CreateLocationQuests>,
    location: String,
    quests: Vec<Quest>,
) -> Result<()> {
    // Validate inputs
    validate_location(&location)?;
    require!(!quests.is_empty(), XploraError::EmptyQuestsArray);
    require!(
        quests.len() <= constants::MAX_QUESTS_PER_LOCATION,
        XploraError::TooManyQuests
    );

    // Validate each quest
    for quest in &quests {
        validate_quest(quest)?;
    }

    // Verify PDA matches location
    let (expected_pda, _bump) = derive_location_quests_pda(ctx.program_id, &location)?;
    require!(
        ctx.accounts.location_quests.key() == expected_pda,
        XploraError::InvalidLocationPDA
    );

    let current_time = get_current_timestamp();
    
    // Initialize location quests account
    let location_quests = &mut ctx.accounts.location_quests;
    location_quests.location = location.clone();
    location_quests.quests = quests;
    location_quests.initialized = true;
    location_quests.created_at = current_time;
    location_quests.updated_at = current_time;
    location_quests.reserved = [0; 6];

    // Update registry counter
    let registry = &mut ctx.accounts.registry;
    registry.total_locations = registry.total_locations
        .checked_add(1)
        .ok_or(XploraError::Overflow)?;

    msg!("Created location quests for: {}", location);
    msg!("Total quests: {}", location_quests.quests.len());
    msg!("Total locations: {}", registry.total_locations);

    Ok(())
}
