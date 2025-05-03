use anchor_lang::prelude::*;

declare_id!("9kb3RJ2M6fqfUR76V8tZbJX3WDQKuTZAspthw23LEbqW");

#[program]
pub mod voting_app_2 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
