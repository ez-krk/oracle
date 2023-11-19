use crate::{
    errors::OracleError,
    state::{oracle::Operator, Oracle},
};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct OperatorAdd<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        mut,
        has_one = owner,
        realloc = Oracle::LEN + (Operator::LEN * (oracle.operators.len() + 1)),
        realloc::zero = false,
        realloc::payer = owner,
        seeds = [b"oracle", owner.key().as_ref()],
        bump = oracle.bump
    )]
    pub oracle: Account<'info, Oracle>,
    #[account(
        constraint = !oracle.operators.iter().any(|i| i.address == operator.key()) @ OracleError::OperatorAlreadyAdded
    )]
    pub operator: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> OperatorAdd<'info> {
    pub fn operator_add(&mut self) -> Result<()> {
        // pub owner: Pubkey,
        // pub value: u64,
        // pub created_at: i64,
        // pub updated_at: i64,
        // pub operators: Vec<Operator>,
        // pub bump: u8,

        let oracle = &mut self.oracle;
        oracle.owner = self.owner.key();

        let operator = Operator {
            address: self.operator.key(),
            value: 0,
            created_at: Clock::get()?.unix_timestamp,
            updated_at: Clock::get()?.unix_timestamp,
        };

        oracle.operators.push(operator);

        Ok(())
    }
}
