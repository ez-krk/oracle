use borsh::{BorshDeserialize, BorshSerialize};
use solana_client::rpc_client::RpcClient;
use solana_program::{instruction::AccountMeta, instruction::Instruction, pubkey::Pubkey};
use solana_sdk::{commitment_config::CommitmentConfig, signature::Signature, signer::Signer};
use std::str::FromStr;

use crate::{client::*, constants::*};

pub fn oracle_update(
    value: u64,
    owner: Pubkey,
    commitment_config: CommitmentConfig,
    wallet_signer: &dyn Signer,
    rpc_client: &RpcClient,
) -> Result<Signature, Box<dyn std::error::Error>> {
    #[derive(BorshDeserialize, BorshSerialize, Debug)]
    pub struct OracleUpdate {
        pub value: u64,
    }
    let (oracle, _) = Pubkey::find_program_address(
        &[b"oracle", owner.as_ref()],
        &Pubkey::from_str(&PROGRAM_ID).unwrap(),
    );

    let data = OracleUpdate { value };

    let instruction = Instruction::new_with_borsh(
        Pubkey::from_str(&PROGRAM_ID).unwrap(),
        &data,
        vec![AccountMeta::new(oracle, false)],
    );
    submit_transaction(rpc_client, wallet_signer, instruction, commitment_config)
}
