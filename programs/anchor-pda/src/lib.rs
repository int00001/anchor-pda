use anchor_lang::prelude::*;

declare_id!("");

#[program]
pub mod anchor_pda {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
