use serde::{Deserialize, Serialize};

use super::validation::ProofNode;

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
pub struct FixtureBatchSummary {
    pub fixture_id: u64,
    pub competition_id: u64,
    pub competition: String,
    pub update_stats: UpdateStats,
    pub update_sub_tree_root: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateStats {
    pub update_count: u64,
    pub min_timestamp: u64,
    pub max_timestamp: u64,
}
