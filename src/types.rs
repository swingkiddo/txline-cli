use serde::{Deserialize, Serialize};

// ── Credentials ─────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Credentials {
    pub network: String,
    pub api_host: String,
    pub jwt: String,
    pub api_token: Option<String>,
}

// ── Auth ────────────────────────────────────────────────────

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

// ── Fixture ─────────────────────────────────────────────────

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fixture {
    pub fixture_id: u64,
    pub competition_id: u64,
    pub home_team: String,
    pub away_team: String,
    pub start_timestamp: u64,
    pub status: String,
}

// ── Odds ────────────────────────────────────────────────────

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OddsPayload {
    pub fixture_id: u64,
    pub market: String,
    pub outcome: String,
    pub odds: f64,
    pub timestamp: u64,
    pub message_id: Option<String>,
}

// ── Scores ──────────────────────────────────────────────────

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Scores {
    pub fixture_id: u64,
    pub home_score: u32,
    pub away_score: u32,
    pub timestamp: u64,
    pub status: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ScoreStat {
    pub key: String,
    pub value: String,
}

// ── Validation ──────────────────────────────────────────────

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FixtureValidation {
    pub fixture_id: u64,
    pub timestamp: u64,
    pub signature: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OddsValidation {
    pub message_id: String,
    pub timestamp: u64,
    pub signature: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScoresStatValidation {
    pub fixture_id: u64,
    pub sequence: u64,
    pub stat_key: String,
    pub stat_key2: Option<String>,
    pub signature: String,
}

// ── SSE ─────────────────────────────────────────────────────

#[derive(Debug, Clone, Deserialize)]
pub struct SseMessage {
    pub id: Option<String>,
    pub event: Option<String>,
    pub data: String,
}

// ── Merkle Proof ────────────────────────────────────────────

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProofNode {
    pub hash: Vec<u8>,
    pub is_right_sibling: bool,
}

// ── On-chain Subscribe ─────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscribeAccounts {
    pub user: String,
    pub pricing_matrix: String,
    pub token_mint: String,
    pub user_token_account: String,
    pub token_treasury_vault: String,
    pub token_treasury_pda: String,
    pub token_program: String,
    pub system_program: String,
    pub associated_token_program: String,
}

// ── Re-exports ──────────────────────────────────────────────

pub use self::{
    ActivationRequest,
    ActivationResponse,
    Credentials,
    Fixture,
    FixtureValidation,
    OddsPayload,
    OddsValidation,
    ProofNode,
    ScoreStat,
    Scores,
    ScoresStatValidation,
    SseMessage,
    SubscribeAccounts,
    TokenResponse,
};
