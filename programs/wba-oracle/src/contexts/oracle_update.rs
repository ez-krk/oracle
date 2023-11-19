use crate::{errors::OracleError, state::Oracle};
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(value: u64)]
pub struct OracleUpdate<'info> {
    #[account(
        mut,
        constraint = oracle.operators.iter().any(|i| i.address == operator.key()) @ OracleError::OperatorDoesNotBelongToThisOracle
    )]
    pub operator: Signer<'info>,
    #[account(
        mut,
        seeds = [b"oracle", oracle.owner.as_ref()],
        bump = oracle.bump
    )]
    pub oracle: Account<'info, Oracle>,
}

impl<'info> OracleUpdate<'info> {
    pub fn oracle_update(&mut self, value: u64) -> Result<()> {
        // pub owner: Pubkey,
        // pub value: u64,
        // pub created_at: i64,
        // pub updated_at: i64,
        // pub operators: Vec<Operator>,
        // pub bump: u8,

        let oracle = &mut self.oracle;

        msg!("hello there");

        let index = oracle
            .operators
            .iter()
            .position(|operator| operator.address == self.operator.key())
            .unwrap();

        oracle.operators[index].value = value;

        let timestamp = Clock::get()?.unix_timestamp;
        oracle.operators[index].updated_at = timestamp;

        let mut sum = 0;
        let mut count = 0;

        let iter = oracle.operators.iter();

        for val in iter {
            sum += val.value;
            count += 1
        }

        oracle.value = value as f64;
        oracle.value = sum as f64 / count as f64;
        oracle.updated_at = timestamp;

        Ok(())
    }
}
