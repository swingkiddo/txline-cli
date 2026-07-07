use std::str::FromStr;

use base64::Engine;
use color_eyre::{Result, eyre::Context};
use reqwest::Client;
use serde_json::Value;
use solana_sdk::hash::Hash;
use solana_sdk::transaction::Transaction;

pub async fn get_recent_blockhash(client: &Client, rpc_url: &str) -> Result<Hash> {
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

pub async fn send_transaction(client: &Client, rpc_url: &str, tx: &Transaction) -> Result<String> {
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
