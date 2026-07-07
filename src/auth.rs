use base64::Engine;
use color_eyre::Result;
use color_eyre::eyre::Context;
#[allow(deprecated)]
use solana_sdk::signature::{Keypair, Signer};

use crate::api::ApiClient;
use crate::types::{ActivationRequest, ActivationResponse, TokenResponse};

/// Perform guest authentication and save the JWT.
pub async fn guest_auth(client: &ApiClient) -> Result<()> {
    let resp: TokenResponse = client
        .post_json("/auth/guest/start", &serde_json::json!({}))
        .await
        .wrap_err("Guest auth request failed")?;

    let mut config = client.config().clone();
    config.jwt = resp.token;
    config.save_credentials()?;

    tracing::info!("Guest authentication successful, JWT saved");
    Ok(())
}

/// Activate an API token using an on-chain transaction signature.
pub async fn activate_token(
    client: &ApiClient,
    tx_sig: &str,
    keypair_path: &str,
    leagues: &[String],
) -> Result<()> {
    let keypair_bytes: String = std::fs::read_to_string(keypair_path)
        .wrap_err_with(|| format!("Failed to read keypair from {keypair_path}"))?
        .trim()
        .to_string();

    let secret: Vec<u8> = serde_json::from_str(&keypair_bytes)
        .wrap_err("Failed to parse keypair JSON array")?;

    let keypair = Keypair::try_from(secret.as_slice())
        .wrap_err("Invalid keypair bytes – expected 64-byte Ed25519 keypair")?;

    let message = format!(
        "{}:{}:{}",
        tx_sig,
        leagues.join(","),
        client.config().jwt
    );

    let signature = keypair.sign_message(message.as_bytes());
    let sig_base64 = base64::engine::general_purpose::STANDARD.encode(signature.as_ref());

    let request = ActivationRequest {
        tx_sig: tx_sig.to_string(),
        signature: sig_base64,
        leagues: leagues.to_vec(),
    };

    let resp: ActivationResponse = client
        .post_json("/api/token/activate", &request)
        .await
        .wrap_err("Token activation request failed")?;

    let mut config = client.config().clone();
    config.api_token = Some(resp.token);
    config.save_credentials()?;

    tracing::info!("API token activated and saved");
    Ok(())
}
