use crate::state::Oracle;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct OracleDelete<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        mut,
        has_one = owner,
        close = owner,
        seeds = [b"oracle", owner.key().as_ref()],
        bump = oracle.bump
    )]
    pub oracle: Account<'info, Oracle>,
}

impl<'info> OracleDelete<'info> {
    pub fn oracle_delete(&mut self) -> Result<()> {
        // pub owner: Pubkey,
        // pub value: u64,
        // pub created_at: i64,
        // pub updated_at: i64,
        // pub operators: Vec<Operator>,
        // pub bump: u8,

        Ok(())
    }
}
