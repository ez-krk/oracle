use crate::{
    errors::OracleError,
    state::{oracle::Operator, Oracle},
};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct OperatorRemove<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        mut,
        has_one = owner,
        realloc = Oracle::LEN + (Operator::LEN * (oracle.operators.len() - 1)),
        realloc::zero = false,
        realloc::payer = owner,
        seeds = [b"oracle", owner.key().as_ref()],
        bump = oracle.bump
    )]
    pub oracle: Account<'info, Oracle>,
    #[account(
        constraint = oracle.operators.iter().any(|i| i.address == operator.key()) @  OracleError::OperatorDoesNotBelongToThisOracle
    )]
    pub operator: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> OperatorRemove<'info> {
    pub fn operator_remove(&mut self) -> Result<()> {
        // pub owner: Pubkey,
        // pub value: u64,
        // pub created_at: i64,
        // pub updated_at: i64,
        // pub operators: Vec<Operator>,
        // pub bump: u8,

        let oracle = &mut self.oracle;

        let index = oracle
            .operators
            .iter()
            .position(|operator| operator.address == self.operator.key())
            .unwrap();

        oracle.operators.remove(index);

        Ok(())
    }
}
