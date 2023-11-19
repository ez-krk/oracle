use anchor_lang::prelude::*;

use crate::constants::*;

#[account]
pub struct Oracle {
    pub verified: bool,
    pub owner: Pubkey,
    pub value: f64,
    pub created_at: i64,
    pub updated_at: i64,
    pub name: String,
    pub operators: Vec<Operator>,
    pub bump: u8,
}

impl Oracle {
    pub const LEN: usize = DISCRIMINATOR_LENGTH
        + BOOL_LENGTH
        + PUBLIC_KEY_LENGTH// owner
        + 8 // value
        + VECTOR_LENGTH_PREFIX // operators vector
        + STRING_LENGTH_PREFIX
        + MAX_NAME_LENGTH
        + TIMESTAMP_LENGTH * 2 // created_at, updated_at
        + BUMP_LENGTH; // bump
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize, PartialEq, Eq)]
pub struct Operator {
    pub address: Pubkey,
    pub value: u64,
    pub created_at: i64,
    pub updated_at: i64,
}

impl Operator {
    pub const LEN: usize = PUBLIC_KEY_LENGTH + 8 + TIMESTAMP_LENGTH * 2;
}
