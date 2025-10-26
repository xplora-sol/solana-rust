use anchor_lang::prelude::*;
use crate::state::*;
use crate::errors::XploraError;
use crate::utils::{validate_quest, get_current_timestamp};

/// Update an existing quest
pub fn update_quest(
    ctx: Context<crate::UpdateQuest>,
    quest_index: u8,
    updated_quest: Quest,
) -> Result<()> {
    let location_quests = &mut ctx.accounts.location_quests;
    
    // Check if location is initialized
    require!(location_quests.initialized, XploraError::NotInitialized);
    
    // Validate quest index
    require!(
        (quest_index as usize) < location_quests.quests.len(),
        XploraError::InvalidQuestIndex
    );

    // Validate updated quest data
    validate_quest(&updated_quest)?;

    // Preserve original creation timestamp
    let original_created_at = location_quests.quests[quest_index as usize].created_at;
    let mut quest_with_timestamp = updated_quest;
    quest_with_timestamp.created_at = original_created_at;

    // Update quest
    location_quests.quests[quest_index as usize] = quest_with_timestamp;
    location_quests.updated_at = get_current_timestamp();

    msg!("Updated quest at index: {}", quest_index);
    msg!("Location: {}", location_quests.location);

    Ok(())
}

