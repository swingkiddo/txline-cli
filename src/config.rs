use std::fs;
use std::path::PathBuf;
use std::str::FromStr;

use color_eyre::Result;
use color_eyre::eyre::Context;
use serde::{Deserialize, Serialize};

use crate::types::Credentials;

#[derive(Debug, Clone)]
pub enum Network {
    Mainnet,
    Devnet,
}

impl Network {
    pub fn api_host(&self) -> &'static str {
        match self {
            Network::Mainnet => "https://txline.txodds.com",
            Network::Devnet => "https://txline-dev.txodds.com",
        }
    }
}

impl std::fmt::Display for Network {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Network::Mainnet => write!(f, "mainnet"),
            Network::Devnet => write!(f, "devnet"),
        }
    }
}

impl FromStr for Network {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "mainnet" => Ok(Network::Mainnet),
            "devnet" => Ok(Network::Devnet),
            _ => Err(format!("Invalid network: {s}. Expected 'mainnet' or 'devnet'")),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Config {
    pub network: Network,
    pub api_host: String,
    pub jwt: String,
    pub api_token: Option<String>,
    pub config_path: Option<PathBuf>,
}

#[derive(Debug, Serialize, Deserialize)]
struct CredentialsFile {
    default: Credentials,
}

impl Config {
    fn config_dir() -> PathBuf {
        directories::BaseDirs::new()
            .map(|d| d.home_dir().join(".txodds"))
            .unwrap_or_else(|| PathBuf::from(".txodds"))
    }

    fn credentials_path() -> PathBuf {
        Self::config_dir().join("credentials.toml")
    }

    pub fn load() -> Result<Self> {
        let _ = dotenvy::dotenv();
        let _ = dotenvy::from_path_override(Self::config_dir().join(".env"));

        let creds_path = Self::credentials_path();

        let mut config = if creds_path.exists() {
            tracing::info!("Loading credentials from {}", creds_path.display());
            let contents =
                fs::read_to_string(&creds_path).wrap_err_with(|| {
                    format!("Failed to read {}", creds_path.display())
                })?;
            let creds_file: CredentialsFile =
                toml::from_str(&contents).wrap_err_with(|| {
                    format!("Failed to parse {}", creds_path.display())
                })?;
            Config::from_credentials(creds_file.default)
        } else {
            tracing::warn!(
                "No credentials file found at {}, using defaults",
                creds_path.display()
            );
            Config::default()
        };

        if let Ok(network) = std::env::var("TXODDS_NETWORK") {
            if let Ok(n) = Network::from_str(&network) {
                config.network = n;
                config.api_host = config.network.api_host().to_string();
            }
        }
        if let Ok(host) = std::env::var("TXODDS_API_HOST") {
            config.api_host = host;
        }
        if let Ok(jwt) = std::env::var("TXODDS_JWT") {
            config.jwt = jwt;
        }
        if let Ok(token) = std::env::var("TXODDS_API_TOKEN") {
            config.api_token = Some(token);
        }

        config.config_path = Some(creds_path);
        Ok(config)
    }

    pub fn from_credentials(creds: Credentials) -> Self {
        let network = Network::from_str(&creds.network).unwrap_or(Network::Devnet);
        let api_host = if creds.api_host.is_empty() {
            network.api_host().to_string()
        } else {
            creds.api_host
        };
        Config {
            network,
            api_host,
            jwt: creds.jwt,
            api_token: creds.api_token,
            config_path: None,
        }
    }

    pub fn save_credentials(&self) -> Result<()> {
        let creds = Credentials {
            network: self.network.to_string(),
            api_host: self.api_host.clone(),
            jwt: self.jwt.clone(),
            api_token: self.api_token.clone(),
        };
        let creds_file = CredentialsFile { default: creds };
        let toml_str =
            toml::to_string_pretty(&creds_file).wrap_err("Failed to serialize credentials")?;

        let dir = Self::config_dir();
        fs::create_dir_all(&dir)
            .wrap_err_with(|| format!("Failed to create config directory {}", dir.display()))?;

        let path = Self::credentials_path();
        fs::write(&path, toml_str)
            .wrap_err_with(|| format!("Failed to write {}", path.display()))?;

        tracing::info!("Credentials saved to {}", path.display());
        Ok(())
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            network: Network::Devnet,
            api_host: Network::Devnet.api_host().to_string(),
            jwt: String::new(),
            api_token: None,
            config_path: None,
        }
    }
}
