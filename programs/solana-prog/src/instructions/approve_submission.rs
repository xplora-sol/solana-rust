use anchor_lang::prelude::*;
use crate::state::*;
use crate::errors::XploraError;

/// Approve a quest submission and distribute rewards
pub fn approve_submission(
    ctx: Context<crate::ApproveSubmission>,
) -> Result<()> {
    let clock = Clock::get()?;
    let submission = &mut ctx.accounts.submission;
    let profile = &mut ctx.accounts.user_profile;
    let location_quests = &ctx.accounts.location_quests;
    
    // Verify submission is pending
    require!(
        submission.status == SubmissionStatus::Pending,
        XploraError::SubmissionNotPending
    );
    
    // Get quest details for reward calculation
    let quest = &location_quests.quests[submission.quest_index as usize];
    
    // Calculate XP reward based on difficulty
    let base_xp = constants::BASE_XP_REWARD;
    let difficulty_multiplier = match quest.difficulty {
        Difficulty::Easy => 1.0,
        Difficulty::Medium => constants::DIFFICULTY_MULTIPLIER_MEDIUM,
        Difficulty::Hard => constants::DIFFICULTY_MULTIPLIER_HARD,
    };
    let xp_reward = (base_xp as f64 * difficulty_multiplier) as u64;
    
    // Calculate token reward based on difficulty and rank tier
    let base_tokens = constants::BASE_TOKEN_REWARD;
    let tier_multiplier = profile.rank_tier.token_multiplier();
    let token_reward = (base_tokens as f64 * difficulty_multiplier * tier_multiplier) as u64;
    
    // Update submission
    submission.status = SubmissionStatus::Approved;
    submission.validator = Some(ctx.accounts.validator.key());
    submission.validated_at = Some(clock.unix_timestamp);
    submission.reward_amount = token_reward;
    
    // Update user profile
    profile.quests_completed = profile.quests_completed.checked_add(1)
        .ok_or(XploraError::Overflow)?;
    profile.experience_points = profile.experience_points.checked_add(xp_reward)
        .ok_or(XploraError::Overflow)?;
    profile.total_tokens_earned = profile.total_tokens_earned.checked_add(token_reward)
        .ok_or(XploraError::Overflow)?;
    profile.last_active = clock.unix_timestamp;
    
    // Update level and rank tier
    let new_level = profile.calculate_level();
    if new_level > profile.level {
        profile.level = new_level;
        profile.rank_tier = RankTier::from_level(new_level);
        msg!("User leveled up to level {}!", new_level);
    }
    
    // Update streak
    update_streak(profile, clock.unix_timestamp)?;
    
    // Track unique location visited (simplified - just increment)
    // TODO: In production, check if this location is new for the user
    if profile.quests_completed == 1 || submission.quest_index == 0 {
        profile.unique_locations = profile.unique_locations.checked_add(1)
            .ok_or(XploraError::Overflow)?;
    }
    
    msg!("Quest approved!");
    msg!("Rewards: {} XP, {} tokens", xp_reward, token_reward);
    msg!("New level: {}, Total XP: {}", profile.level, profile.experience_points);
    
    // Emit reward event
    emit!(QuestRewardEvent {
        user: submission.user,
        submission_pda: submission.key(),
        location: submission.location.clone(),
        quest_index: submission.quest_index,
        xp_reward,
        token_reward,
        new_level: profile.level,
        timestamp: clock.unix_timestamp,
    });
    
    Ok(())
}

/// Helper function to update user streak
fn update_streak(profile: &mut UserProfile, current_timestamp: i64) -> Result<()> {
    const ONE_DAY_SECONDS: i64 = 86_400;
    
    let days_since_last = (current_timestamp - profile.last_quest_date) / ONE_DAY_SECONDS;
    
    if days_since_last == 0 {
        // Same day, no change
    } else if days_since_last == 1 {
        // Next day, increment streak
        profile.current_streak = profile.current_streak.checked_add(1)
            .ok_or(XploraError::Overflow)?;
        
        if profile.current_streak > profile.longest_streak {
            profile.longest_streak = profile.current_streak;
        }
    } else {
        // Streak broken, reset
        profile.current_streak = 1;
    }
    
    profile.last_quest_date = current_timestamp;
    
    Ok(())
}

#[event]
pub struct QuestRewardEvent {
    pub user: Pubkey,
    pub submission_pda: Pubkey,
    pub location: String,
    pub quest_index: u8,
    pub xp_reward: u64,
    pub token_reward: u64,
    pub new_level: u16,
    pub timestamp: i64,
}

