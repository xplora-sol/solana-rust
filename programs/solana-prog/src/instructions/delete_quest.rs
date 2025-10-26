use anchor_lang::prelude::*;
use crate::errors::XploraError;
use crate::utils::get_current_timestamp;

/// Delete a quest from a location
pub fn delete_quest(
    ctx: Context<crate::DeleteQuest>,
    quest_index: u8,
) -> Result<()> {
    let location_quests = &mut ctx.accounts.location_quests;
    
    // Check if location is initialized
    require!(location_quests.initialized, XploraError::NotInitialized);
    
    // Validate quest index
    require!(
        (quest_index as usize) < location_quests.quests.len(),
        XploraError::InvalidQuestIndex
    );

    // Store quest title for logging
    let quest_title = location_quests.quests[quest_index as usize].title.clone();

    // Remove quest from array
    location_quests.quests.remove(quest_index as usize);
    location_quests.updated_at = get_current_timestamp();

    msg!("Deleted quest '{}' at index: {}", quest_title, quest_index);
    msg!("Location: {}", location_quests.location);
    msg!("Remaining quests: {}", location_quests.quests.len());

    Ok(())
}

