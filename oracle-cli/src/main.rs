use borsh::{BorshDeserialize, BorshSerialize};
use solana_client::rpc_client::RpcClient;
use solana_program::{
    instruction::AccountMeta, instruction::Instruction, message::Message, pubkey::Pubkey,
    system_program::ID,
};
use solana_sdk::{
    commitment_config::CommitmentConfig, signature::Signature, signer::Signer,
    transaction::Transaction,
};
use std::{env, str::FromStr};

mod helpers;

const COMMITMENT: CommitmentConfig = CommitmentConfig::finalized();

const URL: &str = "https://api.devnet.solana.com";

const PROGRAM_ID: &str = "WBAVDQ9bRtxDym7G9HvDMSfoYh5i9YR6aiPdHzmSPoa";

pub fn oracle_create(
    operator: &str,
    name: String,
    commitment_config: CommitmentConfig,
    wallet_signer: &dyn Signer,
    rpc_client: &RpcClient,
) -> Result<Signature, Box<dyn std::error::Error>> {
    #[derive(BorshDeserialize, BorshSerialize, Debug)]
    pub struct OracleCreate {
        pub name: String,
    }
    let (oracle, _) = Pubkey::find_program_address(
        &[b"oracle", wallet_signer.pubkey().as_ref()],
        &Pubkey::from_str(&PROGRAM_ID).unwrap(),
    );
    let operator = helpers::parse_pubkey(operator.as_bytes());

    let data = OracleCreate { name };

    let instruction = Instruction::new_with_borsh(
        Pubkey::from_str(&PROGRAM_ID).unwrap(),
        &data,
        vec![
            AccountMeta::new(oracle, false),
            AccountMeta::new_readonly(Pubkey::from(operator), false),
            AccountMeta::new_readonly(Pubkey::from(ID), false),
        ],
    );
    submit_transaction(rpc_client, wallet_signer, instruction, commitment_config)
}

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

pub fn oracle_delete(
    commitment_config: CommitmentConfig,
    wallet_signer: &dyn Signer,
    rpc_client: &RpcClient,
) -> Result<Signature, Box<dyn std::error::Error>> {
    #[derive(BorshDeserialize, BorshSerialize, Debug)]
    pub struct OracleDelete {}
    let (oracle, _) = Pubkey::find_program_address(
        &[b"oracle", wallet_signer.pubkey().as_ref()],
        &Pubkey::from_str(&PROGRAM_ID).unwrap(),
    );

    let data = OracleDelete {};

    let instruction = Instruction::new_with_borsh(
        Pubkey::from_str(&PROGRAM_ID).unwrap(),
        &data,
        vec![AccountMeta::new(oracle, false)],
    );
    submit_transaction(rpc_client, wallet_signer, instruction, commitment_config)
}

pub fn operator_add(
    operator: &str,
    commitment_config: CommitmentConfig,
    wallet_signer: &dyn Signer,
    rpc_client: &RpcClient,
) -> Result<Signature, Box<dyn std::error::Error>> {
    #[derive(BorshDeserialize, BorshSerialize, Debug)]
    pub struct OperatorAdd {}
    let (oracle, _) = Pubkey::find_program_address(
        &[b"oracle", wallet_signer.pubkey().as_ref()],
        &Pubkey::from_str(&PROGRAM_ID).unwrap(),
    );
    let operator = helpers::parse_pubkey(operator.as_bytes());

    let data = OperatorAdd {};

    let instruction = Instruction::new_with_borsh(
        Pubkey::from_str(&PROGRAM_ID).unwrap(),
        &data,
        vec![
            AccountMeta::new(oracle, false),
            AccountMeta::new_readonly(Pubkey::from(operator), false),
            AccountMeta::new_readonly(Pubkey::from(ID), false),
        ],
    );
    submit_transaction(rpc_client, wallet_signer, instruction, commitment_config)
}

pub fn operator_remove(
    operator: &str,
    commitment_config: CommitmentConfig,
    wallet_signer: &dyn Signer,
    rpc_client: &RpcClient,
) -> Result<Signature, Box<dyn std::error::Error>> {
    #[derive(BorshDeserialize, BorshSerialize, Debug)]
    pub struct OperatorRemove {}
    let (oracle, _) = Pubkey::find_program_address(
        &[b"oracle", wallet_signer.pubkey().as_ref()],
        &Pubkey::from_str(&PROGRAM_ID).unwrap(),
    );
    let operator = helpers::parse_pubkey(operator.as_bytes());

    let data = OperatorRemove {};

    let instruction = Instruction::new_with_borsh(
        Pubkey::from_str(&PROGRAM_ID).unwrap(),
        &data,
        vec![
            AccountMeta::new(oracle, false),
            AccountMeta::new_readonly(Pubkey::from(operator), false),
            AccountMeta::new_readonly(Pubkey::from(ID), false),
        ],
    );
    submit_transaction(rpc_client, wallet_signer, instruction, commitment_config)
}

pub fn submit_transaction(
    rpc_client: &RpcClient,
    wallet_signer: &dyn Signer,
    instruction: Instruction,
    commitment_config: CommitmentConfig,
) -> Result<Signature, Box<dyn std::error::Error>> {
    let mut transaction =
        Transaction::new_unsigned(Message::new(&[instruction], Some(&wallet_signer.pubkey())));
    let (recent_blockhash, _fee_calculator) = rpc_client
        .get_recent_blockhash()
        .map_err(|err| format!("error: unable to get recent blockhash: {}", err))?;
    transaction
        .try_sign(&vec![wallet_signer], recent_blockhash)
        .map_err(|err| format!("error: failed to sign transaction: {}", err))?;
    let signature = rpc_client
        .send_and_confirm_transaction_with_spinner_and_commitment(&transaction, commitment_config)
        .map_err(|err| format!("error: send transaction: {}", err))?;
    Ok(signature)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let rpc_client = RpcClient::new(URL);

    let wallet_signer = helpers::keypair(args[2].as_str());

    match args[1].as_str() {
        "oracle_create" => {
            let operator = args[3].as_str();

            let name = args[4].as_str().to_string();
            let signature =
                oracle_create(operator, name, COMMITMENT, &wallet_signer, &rpc_client).unwrap();
            println!(
                "Success! Check out your TX here:
            https://explorer.solana.com/tx/${signature}?cluster=devnet"
            )
        }
        "oracle_update" => {
            let owner: Pubkey = Pubkey::from(helpers::parse_pubkey(args[3].as_str().as_bytes()));

            let value = args[4].as_str().parse::<u64>().unwrap();
            let signature =
                oracle_update(value, owner, COMMITMENT, &wallet_signer, &rpc_client).unwrap();
            println!(
                "Success! Check out your TX here:
          https://explorer.solana.com/tx/${signature}?cluster=devnet"
            )
        }
        "oracle_delete" => {
            let signature = oracle_delete(COMMITMENT, &wallet_signer, &rpc_client).unwrap();
            println!(
                "Success! Check out your TX here:
          https://explorer.solana.com/tx/${signature}?cluster=devnet"
            )
        }
        "operator_add" => {
            let operator = args[3].as_str();

            let signature =
                operator_add(operator, COMMITMENT, &wallet_signer, &rpc_client).unwrap();
            println!(
                "Success! Check out your TX here:
          https://explorer.solana.com/tx/${signature}?cluster=devnet"
            )
        }
        "operator_remove" => {
            let operator = args[3].as_str();

            let signature =
                operator_remove(operator, COMMITMENT, &wallet_signer, &rpc_client).unwrap();
            println!(
                "Success! Check out your TX here:
          https://explorer.solana.com/tx/${signature}?cluster=devnet"
            )
        }
        _ => println!("something went wrong !"),
    }
}
