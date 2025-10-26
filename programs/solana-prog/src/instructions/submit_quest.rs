use anchor_lang::prelude::*;
use crate::state::*;
use crate::errors::XploraError;

/// Submit a quest completion with IPFS photo hash
pub fn submit_quest_completion(
    ctx: Context<crate::SubmitQuestCompletion>,
    location: String,
    quest_index: u8,
    ipfs_hash: String,
    description: String,
) -> Result<()> {
    let clock = Clock::get()?;
    let submission = &mut ctx.accounts.submission;
    let location_quests = &ctx.accounts.location_quests;
    
    // Validate quest exists
    require!(
        (quest_index as usize) < location_quests.quests.len(),
        XploraError::InvalidQuestIndex
    );
    
    // Validate IPFS hash format (basic check)
    require!(
        ipfs_hash.len() >= 46 && ipfs_hash.starts_with("Qm"),
        XploraError::InvalidIpfsHash
    );
    
    // Validate description length
    require!(
        description.len() <= QuestSubmission::MAX_DESCRIPTION_LEN,
        XploraError::InvalidDescription
    );
    
    // Initialize submission
    submission.user = ctx.accounts.user.key();
    submission.location = location.clone();
    submission.quest_index = quest_index;
    submission.ipfs_hash = ipfs_hash.clone();
    submission.description = description;
    submission.submitted_at = clock.unix_timestamp;
    submission.status = SubmissionStatus::Pending;
    submission.validator = None;
    submission.validated_at = None;
    submission.reward_amount = 0;
    submission.attempt_number = 1; // Can be enhanced to track multiple attempts
    submission.bump = ctx.bumps.submission;
    
    // Update user profile attempts
    let profile = &mut ctx.accounts.user_profile;
    profile.quests_attempted = profile.quests_attempted.checked_add(1)
        .ok_or(XploraError::Overflow)?;
    profile.last_active = clock.unix_timestamp;
    
    msg!("Quest submission created for user: {}", ctx.accounts.user.key());
    msg!("Location: {}, Quest: {}", location, quest_index);
    msg!("IPFS Hash: {}", ipfs_hash);
    
    // Emit event for backend monitoring
    emit!(QuestSubmissionEvent {
        user: ctx.accounts.user.key(),
        submission_pda: submission.key(),
        location: location.clone(),
        quest_index,
        ipfs_hash,
        timestamp: clock.unix_timestamp,
    });
    
    Ok(())
}

#[event]
pub struct QuestSubmissionEvent {
    pub user: Pubkey,
    pub submission_pda: Pubkey,
    pub location: String,
    pub quest_index: u8,
    pub ipfs_hash: String,
    pub timestamp: i64,
}

