use anchor_lang::prelude::*;
use crate::state::*;
use crate::errors::XploraError;

/// Reject a quest submission
pub fn reject_submission(
    ctx: Context<crate::RejectSubmission>,
    reason: String,
) -> Result<()> {
    let clock = Clock::get()?;
    let submission = &mut ctx.accounts.submission;
    
    // Verify submission is pending
    require!(
        submission.status == SubmissionStatus::Pending,
        XploraError::SubmissionNotPending
    );
    
    // Validate reason length
    require!(
        !reason.is_empty() && reason.len() <= 200,
        XploraError::InvalidDescription
    );
    
    // Update submission
    submission.status = SubmissionStatus::Rejected;
    submission.validator = Some(ctx.accounts.validator.key());
    submission.validated_at = Some(clock.unix_timestamp);
    
    msg!("Quest submission rejected");
    msg!("Reason: {}", reason);
    
    // Emit rejection event
    emit!(QuestRejectionEvent {
        user: submission.user,
        submission_pda: submission.key(),
        location: submission.location.clone(),
        quest_index: submission.quest_index,
        reason,
        timestamp: clock.unix_timestamp,
    });
    
    Ok(())
}

#[event]
pub struct QuestRejectionEvent {
    pub user: Pubkey,
    pub submission_pda: Pubkey,
    pub location: String,
    pub quest_index: u8,
    pub reason: String,
    pub timestamp: i64,
}

