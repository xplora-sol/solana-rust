use anchor_lang::prelude::*;
use crate::state::*;
use crate::errors::XploraError;

/// Initialize a user profile
pub fn initialize_user_profile(
    ctx: Context<crate::InitializeUserProfile>,
    username: String,
) -> Result<()> {
    let clock = Clock::get()?;
    let profile = &mut ctx.accounts.user_profile;
    
    // Validate username
    require!(
        username.len() <= UserProfile::MAX_USERNAME_LEN && !username.is_empty(),
        XploraError::InvalidUsername
    );
    
    // Initialize profile
    profile.user = ctx.accounts.user.key();
    profile.username = username.clone();
    profile.created_at = clock.unix_timestamp;
    profile.last_active = clock.unix_timestamp;
    profile.quests_completed = 0;
    profile.quests_attempted = 0;
    profile.experience_points = 0;
    profile.level = 0;
    profile.total_tokens_earned = 0;
    profile.unique_locations = 0;
    profile.current_streak = 0;
    profile.longest_streak = 0;
    profile.last_quest_date = 0;
    profile.achievements = 0; // No achievements initially
    profile.rank_tier = RankTier::Bronze;
    profile.bump = ctx.bumps.user_profile;
    
    msg!("User profile created for: {}", ctx.accounts.user.key());
    msg!("Username: {}", username);
    
    // Emit profile creation event
    emit!(ProfileCreatedEvent {
        user: ctx.accounts.user.key(),
        username,
        timestamp: clock.unix_timestamp,
    });
    
    Ok(())
}

#[event]
pub struct ProfileCreatedEvent {
    pub user: Pubkey,
    pub username: String,
    pub timestamp: i64,
}

