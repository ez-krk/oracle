use crate::{
    errors::OracleError,
    state::{oracle::Operator, Oracle},
};
use anchor_lang::prelude::*;
use std::collections::BTreeMap;

#[derive(Accounts)]
#[instruction(name: String)]
pub struct OracleCreate<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        init,
        payer = owner,
        seeds = [b"oracle", owner.key().as_ref()],
        space = Oracle::LEN + Operator::LEN,
        bump
    )]
    pub oracle: Account<'info, Oracle>,
    #[account(
        constraint = !oracle.operators.iter().any(|i| i.address == operator.key()) @ OracleError::OperatorAlreadyAdded
    )]
    pub operator: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> OracleCreate<'info> {
    pub fn oracle_create(&mut self, bumps: &BTreeMap<String, u8>, name: String) -> Result<()> {
        // pub owner: Pubkey,
        // pub value: u64,
        // pub created_at: i64,
        // pub updated_at: i64,
        // pub operators: Vec<Operator>,
        // pub bump: u8,

        let oracle = &mut self.oracle;
        oracle.verified = false;
        oracle.owner = self.owner.key();
        oracle.value = 0.0;
        oracle.name = name;
        oracle.operators = Vec::new();
        let timestamp = Clock::get()?.unix_timestamp;
        oracle.created_at = timestamp;
        oracle.updated_at = timestamp;
        let operator = Operator {
            address: self.operator.key(),
            value: 0,
            created_at: timestamp,
            updated_at: timestamp,
        };
        oracle.operators.push(operator);
        oracle.bump = *bumps.get("oracle").unwrap();

        Ok(())
    }
}
