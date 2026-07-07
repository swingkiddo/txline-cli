use std::str::FromStr;

use base64::Engine;
use color_eyre::{Result, eyre::Context};
use reqwest::Client;
use serde_json::Value;
#[allow(deprecated)]
use solana_sdk::{
    hash::Hash,
    instruction::{AccountMeta, Instruction},
    message::Message,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::Transaction,
};

use crate::config::Network;
use crate::types::SubscribeAccounts;

const SUBSCRIBE_DISCRIMINATOR: [u8; 8] = [254, 28, 191, 138, 156, 179, 183, 53];

fn program_id(network: &Network) -> Pubkey {
    match network {
        Network::Mainnet => {
            Pubkey::from_str("9ExbZjAapQww1vfcisDmrngPinHTEfpjYRWMunJgcKaA").unwrap()
        }
        Network::Devnet => {
            Pubkey::from_str("6pW64gN1s2uqjHkn1unFeEjAwJkPGHoppGvS715wyP2J").unwrap()
        }
    }
}

fn token_mint(network: &Network) -> Pubkey {
    match network {
        Network::Mainnet => {
            Pubkey::from_str("Zhw9TVKp68a1QrftncMSd6ELXKDtpVMNuMGr1jNwdeL").unwrap()
        }
        Network::Devnet => {
            Pubkey::from_str("4Zao8ocPhmMgq7PdsYWyxvqySMGx7xb9cMftPMkEokRG").unwrap()
        }
    }
}

fn find_associated_token_address(
    owner: &Pubkey,
    mint: &Pubkey,
    token_program: &Pubkey,
    associated_token_program: &Pubkey,
) -> Pubkey {
    let seeds = &[owner.as_ref(), token_program.as_ref(), mint.as_ref()];
    Pubkey::find_program_address(seeds, associated_token_program).0
}

const TOKEN_2022: &str = "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb";
const ASSOCIATED_TOKEN: &str = "ATokenGPvbdGVxr1b2hvZbsiqW5xr25e9TcAaCnjTXHp1";
const SYSTEM_PROGRAM: &str = "11111111111111111111111111111111";

pub fn derive_subscribe_accounts(
    user_pubkey: &Pubkey,
    network: &Network,
) -> SubscribeAccounts {
    let pid = program_id(network);
    let mint = token_mint(network);
    let token_prog = Pubkey::from_str(TOKEN_2022).unwrap();
    let atp = Pubkey::from_str(ASSOCIATED_TOKEN).unwrap();

    let (token_treasury_pda, _) =
        Pubkey::find_program_address(&[b"token_treasury_v2"], &pid);
    let token_treasury_vault =
        find_associated_token_address(&token_treasury_pda, &mint, &token_prog, &atp);
    let (pricing_matrix, _) =
        Pubkey::find_program_address(&[b"pricing_matrix"], &pid);
    let user_token_account =
        find_associated_token_address(user_pubkey, &mint, &token_prog, &atp);

    SubscribeAccounts {
        user: *user_pubkey,
        pricing_matrix,
        token_mint: mint,
        user_token_account,
        token_treasury_vault,
        token_treasury_pda,
        token_program: token_prog,
        system_program: Pubkey::from_str(SYSTEM_PROGRAM).unwrap(),
        associated_token_program: atp,
    }
}

pub fn build_subscribe_instruction(
    accounts: &SubscribeAccounts,
    network: &Network,
    service_level_id: u32,
    weeks: u32,
) -> Instruction {
    let pid = program_id(network);

    let mut data = Vec::with_capacity(11);
    data.extend_from_slice(&SUBSCRIBE_DISCRIMINATOR);
    data.extend_from_slice(&(service_level_id as u16).to_le_bytes());
    data.push(weeks as u8);

    Instruction {
        program_id: pid,
        accounts: vec![
            AccountMeta::new(accounts.user, true),
            AccountMeta::new(accounts.pricing_matrix, false),
            AccountMeta::new_readonly(accounts.token_mint, false),
            AccountMeta::new(accounts.user_token_account, false),
            AccountMeta::new(accounts.token_treasury_vault, false),
            AccountMeta::new(accounts.token_treasury_pda, false),
            AccountMeta::new_readonly(accounts.token_program, false),
            AccountMeta::new_readonly(accounts.system_program, false),
            AccountMeta::new_readonly(accounts.associated_token_program, false),
        ],
        data,
    }
}

fn read_keypair(path: &str) -> Result<Keypair> {
    let json = std::fs::read_to_string(path)
        .wrap_err_with(|| format!("Failed to read keypair from {path}"))?;
    let secret: Vec<u8> = serde_json::from_str(json.trim())
        .wrap_err("Failed to parse keypair JSON array")?;
    Keypair::try_from(secret.as_slice())
        .wrap_err("Invalid keypair bytes – expected 64-byte Ed25519 keypair")
}

async fn get_recent_blockhash(client: &Client, rpc_url: &str) -> Result<Hash> {
    let resp: Value = client
        .post(rpc_url)
        .json(&serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "getLatestBlockhash",
            "params": [{"commitment": "confirmed"}]
        }))
        .send()
        .await
        .wrap_err("Failed to get recent blockhash")?
        .json()
        .await
        .wrap_err("Failed to parse blockhash response")?;

    let blockhash_str = resp["result"]["value"]["blockhash"]
        .as_str()
        .ok_or_else(|| color_eyre::eyre::eyre!("Missing blockhash in response"))?;

    Hash::from_str(blockhash_str).wrap_err("Invalid blockhash")
}

async fn send_transaction(client: &Client, rpc_url: &str, tx: &Transaction) -> Result<String> {
    let tx_bytes = bincode::serialize(tx).wrap_err("Failed to serialize transaction")?;
    let tx_b64 = base64::engine::general_purpose::STANDARD.encode(&tx_bytes);

    let resp: Value = client
        .post(rpc_url)
        .json(&serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "sendTransaction",
            "params": [tx_b64, {
                "encoding": "base64",
                "commitment": "confirmed",
                "maxRetries": 3
            }]
        }))
        .send()
        .await
        .wrap_err("Failed to send transaction")?
        .json()
        .await
        .wrap_err("Failed to parse send transaction response")?;

    if let Some(err) = resp["error"].as_object() {
        let msg = err
            .get("message")
            .and_then(|m| m.as_str())
            .unwrap_or("unknown error");
        color_eyre::eyre::bail!("Transaction failed: {msg}");
    }

    resp["result"]
        .as_str()
        .map(|s| s.to_string())
        .ok_or_else(|| color_eyre::eyre::eyre!("Missing transaction signature in response"))
}

pub async fn subscribe_onchain(
    keypair_path: &str,
    network: &Network,
    service_level_id: u32,
    weeks: u32,
    rpc_url: &str,
) -> Result<String> {
    let keypair = read_keypair(keypair_path)?;
    let user_pubkey = keypair.pubkey();
    let accounts = derive_subscribe_accounts(&user_pubkey, network);
    let ix = build_subscribe_instruction(&accounts, network, service_level_id, weeks);

    let client = Client::new();
    let recent_blockhash = get_recent_blockhash(&client, rpc_url).await?;

    let message = Message::new(&[ix], Some(&user_pubkey));
    let tx = Transaction::new(&[&keypair], message, recent_blockhash);

    let tx_sig = send_transaction(&client, rpc_url, &tx).await?;
    tracing::info!("Subscribe transaction sent: {tx_sig}");
    Ok(tx_sig)
}
