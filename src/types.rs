use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Fixture {
    pub fixture_id: u64,
    pub start_time: u64,
    pub participant1: String,
    pub participant2: String,
    pub participant1_is_home: bool,
    pub competition: String,
    pub competition_id: u64,
    pub participant1_id: u64,
    pub participant2_id: u64,
    pub fixture_group_id: u64,
    pub ts: u64,
}

// ── Odds ────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct OddsPayload {
    pub fixture_id: u64,
    pub message_id: String,
    pub ts: u64,
    pub bookmaker: String,
    pub bookmaker_id: u64,
    pub super_odds_type: String,
    pub game_state: Option<String>,
    pub in_running: bool,
    pub market_parameters: Option<String>,
    pub market_period: Option<String>,
    pub price_names: Vec<String>,
    pub prices: Vec<f64>,
    pub pct: Vec<String>,
}

// ── Scores ──────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Scores {
    pub fixture_id: u64,
    pub game_state: String,
    pub start_time: u64,
    pub is_team: bool,
    pub fixture_group_id: u64,
    pub competition_id: u64,
    pub country_id: u64,
    pub sport_id: u64,
    pub participant1_is_home: bool,
    pub participant2_id: u64,
    pub participant1_id: u64,
    pub action: String,
    pub id: u64,
    pub ts: u64,
    pub connection_id: u64,
    pub seq: u64,
    pub stats: HashMap<String, u32>,
    pub clock: Option<SoccerFixtureClock>,
    pub score: Option<SoccerFixtureScore>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoreStat {
    pub key: u64,
    pub value: u64,
    pub period: u64,
}

// ── Soccer Types ────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SoccerScore {
    pub goals: Option<u32>,
    pub yellow_cards: Option<u32>,
    pub red_cards: Option<u32>,
    pub corners: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SoccerTotalScore {
    pub h1: Option<SoccerScore>,
    pub ht: Option<SoccerScore>,
    pub h2: Option<SoccerScore>,
    pub et1: Option<SoccerScore>,
    pub et2: Option<SoccerScore>,
    pub pe: Option<SoccerScore>,
    pub et_total: Option<SoccerScore>,
    pub total: Option<SoccerScore>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SoccerFixtureScore {
    pub participant1: Option<SoccerTotalScore>,
    pub participant2: Option<SoccerTotalScore>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SoccerFixtureClock {
    pub running: bool,
    pub seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SoccerData {
    pub fixture_id: Option<u64>,
    pub competition_id: Option<u64>,
    pub competition: Option<String>,
    pub participant1: Option<String>,
    pub participant2: Option<String>,
    pub participant1_id: Option<u64>,
    pub participant2_id: Option<u64>,
    pub participant1_is_home: Option<bool>,
    pub start_time: Option<u64>,
    pub game_state: Option<String>,
    pub score: Option<SoccerFixtureScore>,
    pub clock: Option<SoccerFixtureClock>,
    pub stats: Option<HashMap<String, u32>>,
    pub player_stats: Option<HashMap<u64, SoccerPlayerStat>>,
    pub update_reference: Option<SoccerUpdateReference>,
    pub fixture_group_id: Option<u64>,
    pub country_id: Option<u64>,
    pub sport_id: Option<u64>,
    pub action: Option<String>,
    pub id: Option<u64>,
    pub ts: Option<u64>,
    pub connection_id: Option<u64>,
    pub seq: Option<u64>,
    pub is_team: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SoccerUpdateReference {
    pub id: Option<u64>,
    pub ts: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SoccerPlayerStat {
    pub goals: Option<u32>,
    pub yellow_cards: Option<u32>,
    pub red_cards: Option<u32>,
}

// ── Validation ──────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FixtureValidation {
    pub snapshot: Fixture,
    pub summary: FixtureBatchSummary,
    pub sub_tree_proof: Vec<ProofNode>,
    pub main_tree_proof: Vec<ProofNode>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OddsValidation {
    pub odds: OddsPayload,
    pub summary: OddsBatchSummary,
    pub sub_tree_proof: Vec<ProofNode>,
    pub main_tree_proof: Vec<ProofNode>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScoresStatValidation {
    pub ts: u64,
    pub stat_to_prove: ScoreStat,
    pub event_stat_root: Vec<u8>,
    pub summary: ScoresBatchSummary,
    pub stat_proof: Vec<ProofNode>,
    pub sub_tree_proof: Vec<ProofNode>,
    pub main_tree_proof: Vec<ProofNode>,
    pub stat_to_prove2: Option<ScoreStat>,
    pub stat_proof2: Option<Vec<ProofNode>>,
}

// ── Batch Summary Types ─────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FixtureBatchSummary {
    pub fixture_id: u64,
    pub competition_id: u64,
    pub competition: String,
    pub update_stats: UpdateStats,
    pub update_sub_tree_root: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OddsBatchSummary {
    pub fixture_id: u64,
    pub update_stats: UpdateStats,
    pub odds_sub_tree_root: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScoresBatchSummary {
    pub fixture_id: u64,
    pub update_stats: ScoresUpdateStats,
    pub event_stats_sub_tree_root: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateStats {
    pub update_count: u64,
    pub min_timestamp: u64,
    pub max_timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScoresUpdateStats {
    pub update_count: u64,
    pub min_timestamp: u64,
    pub max_timestamp: u64,
}

// ── Merkle Proof ────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProofNode {
    pub hash: Vec<u8>,
    pub is_right_sibling: bool,
}

// ── SSE ─────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SseMessage {
    pub id: Option<String>,
    pub event: Option<String>,
    pub data: String,
}

// ── On-chain Subscribe ─────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscribeAccounts {
    pub user: solana_sdk::pubkey::Pubkey,
    pub pricing_matrix: solana_sdk::pubkey::Pubkey,
    pub token_mint: solana_sdk::pubkey::Pubkey,
    pub user_token_account: solana_sdk::pubkey::Pubkey,
    pub token_treasury_vault: solana_sdk::pubkey::Pubkey,
    pub token_treasury_pda: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub associated_token_program: solana_sdk::pubkey::Pubkey,
}
