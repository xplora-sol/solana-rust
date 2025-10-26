use anchor_lang::prelude::*;
use crate::state::*;

/// Initialize the quest registry
pub fn initialize(ctx: Context<crate::Initialize>, authority: Pubkey) -> Result<()> {
    let registry = &mut ctx.accounts.registry;
    
    // Set initial values
    registry.authority = authority;
    registry.total_locations = 0;
    registry.version = constants::PROGRAM_VERSION;
    registry.reserved = [0; 7];
    
    msg!("Quest Registry initialized with authority: {}", authority);
    msg!("Program version: {}", constants::PROGRAM_VERSION);
    
    Ok(())
}
