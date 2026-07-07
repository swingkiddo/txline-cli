use serde::{Deserialize, Serialize};

use super::fixtures::UpdateStats;
use super::validation::ProofNode;

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
pub struct OddsBatchSummary {
    pub fixture_id: u64,
    pub update_stats: UpdateStats,
    pub odds_sub_tree_root: Vec<u8>,
}
