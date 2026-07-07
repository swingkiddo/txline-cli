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

#[derive(Debug, Clone, borsh::BorshSerialize, borsh::BorshDeserialize)]
pub struct OddsPayloadBorsh {
    pub fixture_id: i64,
    pub message_id: String,
    pub ts: i64,
    pub bookmaker: String,
    pub bookmaker_id: i32,
    pub super_odds_type: String,
    pub game_state: Option<String>,
    pub in_running: bool,
    pub market_parameters: Option<String>,
    pub market_period: Option<String>,
    pub price_names: Vec<String>,
    pub prices: Vec<i32>,
}

impl From<&OddsPayload> for OddsPayloadBorsh {
    fn from(o: &OddsPayload) -> Self {
        Self {
            fixture_id: o.fixture_id as i64,
            message_id: o.message_id.clone(),
            ts: o.ts as i64,
            bookmaker: o.bookmaker.clone(),
            bookmaker_id: o.bookmaker_id as i32,
            super_odds_type: o.super_odds_type.clone(),
            game_state: o.game_state.clone(),
            in_running: o.in_running,
            market_parameters: o.market_parameters.clone(),
            market_period: o.market_period.clone(),
            price_names: o.price_names.clone(),
            prices: o.prices.iter().map(|&p| p as i32).collect(),
        }
    }
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

#[derive(Debug, Clone, borsh::BorshSerialize, borsh::BorshDeserialize)]
pub struct OddsBatchSummaryBorsh {
    pub fixture_id: i64,
    pub update_stats: super::fixtures::UpdateStatsBorsh,
    pub odds_sub_tree_root: [u8; 32],
}

impl From<&OddsBatchSummary> for OddsBatchSummaryBorsh {
    fn from(s: &OddsBatchSummary) -> Self {
        let mut root = [0u8; 32];
        if s.odds_sub_tree_root.len() == 32 {
            root.copy_from_slice(&s.odds_sub_tree_root);
        }
        Self {
            fixture_id: s.fixture_id as i64,
            update_stats: super::fixtures::UpdateStatsBorsh::from(&s.update_stats),
            odds_sub_tree_root: root,
        }
    }
}
