use std::str::FromStr;

use anyhow::anyhow;
use num_format::{Locale, ToFormattedString};
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::model::prelude::Activity;
use serenity::prelude::*;
use shuttle_secrets::SecretStore;
use solana_client::rpc_client::RpcClient;
use solana_program::pubkey::Pubkey;
use std::env;
use tokio::time;
use tracing::{error, info};
struct Bot;

mod client;
mod constants;
mod contexts;
mod helpers;

use crate::constants::*;
use crate::contexts::*;

#[async_trait]
impl EventHandler for Bot {
    async fn ready(&self, ctx: Context, ready: Ready) {
        let mut interval = time::interval(std::time::Duration::from_secs(60));

        let client = reqwest::Client::new();

        info!("{} is connected!", ready.user.name);

        let gecko_base = "https://api.coingecko.com/api/v3";

        // change this list
        let crypto = "solana";
        let fiat = "usd";

        let gecko_price = format!(
            "{}/simple/price?ids={}&vs_currencies={}",
            gecko_base, crypto, fiat
        );

        tokio::spawn(async move {
            loop {
                interval.tick().await;
                let price = fetch(&client, &gecko_price, crypto, fiat).await;
                println!("{}", price);
                ctx.set_activity(Activity::watching(format!("$ {}", &price)))
                    .await;
                let rpc_client = RpcClient::new(RPC_ENDPOINT);
                let wallet_signer = helpers::keypair();
                let owner: Pubkey =
                    Pubkey::from_str(env::var("KEYPAIR").unwrap().as_str()).unwrap();
                let value = price.as_str().parse::<u64>().unwrap();
                let signature =
                    oracle_update(value, owner, COMMITMENT, &wallet_signer, &rpc_client).unwrap();
                println!(
                    "Success! Check out your TX here:
              https://explorer.solana.com/tx/${signature}?cluster=devnet"
                )
            }
        });
    }
}

#[shuttle_runtime::main]
async fn serenity(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> shuttle_serenity::ShuttleSerenity {
    let token = if let Some(token) = secret_store.get("DISCORD_TOKEN") {
        token
    } else {
        return Err(anyhow!("'DISCORD_TOKEN' was not found").into());
    };

    let keypair = if let Some(keypair) = secret_store.get("KEYPAIR") {
        std::env::set_var("KEYPAIR", &keypair);
        keypair
    } else {
        return Err(anyhow!("'KEYPAIR' not found").into());
    };

    let owner = if let Some(owner) = secret_store.get("OWNER") {
        std::env::set_var("OWNER", &owner);
        owner
    } else {
        return Err(anyhow!("'OWNER' not found").into());
    };

    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let client = Client::builder(&token, intents)
        .event_handler(Bot)
        .await
        .expect("Err creating client");

    Ok(client.into())
}

async fn fetch(client: &reqwest::Client, url: &String, crypto: &str, fiat: &str) -> String {
    let body = client
        .get(url)
        .send()
        .await
        .unwrap()
        .json::<serde_json::Value>()
        .await
        .unwrap();
    let data = body.get(crypto).unwrap();
    data[format!("{}", fiat)]
        .as_u64()
        .unwrap()
        .to_formatted_string(&Locale::en)
}
