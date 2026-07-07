use color_eyre::{Result, eyre::Context};
use solana_sdk::signature::Keypair;

pub fn read_keypair(path: &str) -> Result<Keypair> {
    let json = std::fs::read_to_string(path)
        .wrap_err_with(|| format!("Failed to read keypair from {path}"))?;
    let secret: Vec<u8> = serde_json::from_str(json.trim())
        .wrap_err("Failed to parse keypair JSON array")?;
    Keypair::try_from(secret.as_slice())
        .wrap_err("Invalid keypair bytes – expected 64-byte Ed25519 keypair")
}
