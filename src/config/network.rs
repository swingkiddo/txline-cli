use std::str::FromStr;

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
