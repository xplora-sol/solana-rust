use anchor_lang::prelude::*;

declare_id!("3rD6xKajAwvt8xbN5tkSSM8CvftGDs5x9jinkCK4BCCj");

#[program]
pub mod solana_prog {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
