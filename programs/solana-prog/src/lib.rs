use anchor_lang::prelude::*;

// Module declarations
pub mod errors;
pub mod state;
pub mod utils;
pub mod instructions;

// Re-exports for convenience
pub use errors::XploraError;
pub use state::*;

declare_id!("3rD6xKajAwvt8xbN5tkSSM8CvftGDs5x9jinkCK4BCCj");

#[program]
pub mod xplora_quests {
    use super::*;

    /// Initialize the quest registry with an authority
    pub fn initialize(ctx: Context<Initialize>, authority: Pubkey) -> Result<()> {
        instructions::initialize::initialize(ctx, authority)
    }

    /// Create quests for a new location
    pub fn create_location_quests(
        ctx: Context<CreateLocationQuests>,
        location: String,
        quests: Vec<Quest>,
    ) -> Result<()> {
        instructions::create_location::create_location_quests(ctx, location, quests)
    }

    /// Add a new quest to an existing location
    pub fn add_quest_to_location(
        ctx: Context<AddQuestToLocation>,
        quest: Quest,
    ) -> Result<()> {
        instructions::add_quest::add_quest_to_location(ctx, quest)
    }

    /// Update an existing quest
    pub fn update_quest(
        ctx: Context<UpdateQuest>,
        quest_index: u8,
        updated_quest: Quest,
    ) -> Result<()> {
        instructions::update_quest::update_quest(ctx, quest_index, updated_quest)
    }

    /// Delete a quest from a location
    pub fn delete_quest(
        ctx: Context<DeleteQuest>,
        quest_index: u8,
    ) -> Result<()> {
        instructions::delete_quest::delete_quest(ctx, quest_index)
    }
}

// Context structs need to be at crate root for Anchor to find them
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = payer,
        space = QuestRegistry::LEN,
        seeds = [b"quest_registry"],
        bump
    )]
    pub registry: Account<'info, QuestRegistry>,
    
    #[account(mut)]
    pub payer: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(location: String)]
pub struct CreateLocationQuests<'info> {
    #[account(
        mut,
        seeds = [b"quest_registry"],
        bump,
        has_one = authority @ XploraError::Unauthorized
    )]
    pub registry: Account<'info, QuestRegistry>,
    
    #[account(
        init,
        payer = authority,
        space = LocationQuests::space(),
        seeds = [b"location_quests", location.as_bytes()],
        bump
    )]
    pub location_quests: Account<'info, LocationQuests>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddQuestToLocation<'info> {
    #[account(
        mut,
        seeds = [b"quest_registry"],
        bump,
        has_one = authority @ XploraError::Unauthorized
    )]
    pub registry: Account<'info, QuestRegistry>,
    
    #[account(
        mut,
        constraint = location_quests.initialized @ XploraError::NotInitialized
    )]
    pub location_quests: Account<'info, LocationQuests>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct UpdateQuest<'info> {
    #[account(
        mut,
        seeds = [b"quest_registry"],
        bump,
        has_one = authority @ XploraError::Unauthorized
    )]
    pub registry: Account<'info, QuestRegistry>,
    
    #[account(
        mut,
        constraint = location_quests.initialized @ XploraError::NotInitialized
    )]
    pub location_quests: Account<'info, LocationQuests>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct DeleteQuest<'info> {
    #[account(
        mut,
        seeds = [b"quest_registry"],
        bump,
        has_one = authority @ XploraError::Unauthorized
    )]
    pub registry: Account<'info, QuestRegistry>,
    
    #[account(
        mut,
        constraint = location_quests.initialized @ XploraError::NotInitialized
    )]
    pub location_quests: Account<'info, LocationQuests>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
}