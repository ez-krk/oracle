use anchor_lang::prelude::*;

mod constants;
mod contexts;
mod errors;
mod state;

use contexts::*;

declare_id!("WBAVDQ9bRtxDym7G9HvDMSfoYh5i9YR6aiPdHzmSPoa");

#[program]
pub mod wba_oracle {
    use super::*;

    pub fn oracle_create(ctx: Context<OracleCreate>, name: String) -> Result<()> {
        ctx.accounts.oracle_create(&ctx.bumps, name)
    }

    pub fn oracle_update(ctx: Context<OracleUpdate>, value: u64) -> Result<()> {
        ctx.accounts.oracle_update(value)
    }

    pub fn oracle_delete(ctx: Context<OracleDelete>) -> Result<()> {
        ctx.accounts.oracle_delete()
    }

    pub fn operator_add(ctx: Context<OperatorAdd>) -> Result<()> {
        ctx.accounts.operator_add()
    }

    pub fn operator_remove(ctx: Context<OperatorRemove>) -> Result<()> {
        ctx.accounts.operator_remove()
    }
}
