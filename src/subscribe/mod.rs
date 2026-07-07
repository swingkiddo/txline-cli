mod accounts;
mod instruction;
mod keypair;
mod rpc;

use color_eyre::Result;
use solana_sdk::signature::Signer;

use crate::config::Network;

pub async fn subscribe_onchain(
    keypair_path: &str,
    network: &Network,
    service_level_id: u32,
    weeks: u32,
    rpc_url: &str,
) -> Result<String> {
    let keypair = keypair::read_keypair(keypair_path)?;
    let user_pubkey = keypair.pubkey();
    let accounts = accounts::derive_subscribe_accounts(&user_pubkey, network);
    let ix = instruction::build_subscribe_instruction(&accounts, network, service_level_id, weeks);

    let client = reqwest::Client::new();
    let recent_blockhash = rpc::get_recent_blockhash(&client, rpc_url).await?;

    let message = solana_sdk::message::Message::new(&[ix], Some(&user_pubkey));
    let tx = solana_sdk::transaction::Transaction::new(&[&keypair], message, recent_blockhash);

    let tx_sig = rpc::send_transaction(&client, rpc_url, &tx).await?;
    tracing::info!("Subscribe transaction sent: {tx_sig}");
    Ok(tx_sig)
}
