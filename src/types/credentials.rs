use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Credentials {
    pub network: String,
    pub api_host: String,
    pub jwt: String,
    pub api_token: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TokenResponse {
    pub token: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivationRequest {
    pub tx_sig: String,
    pub signature: String,
    pub leagues: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ActivationResponse {
    pub token: String,
}
