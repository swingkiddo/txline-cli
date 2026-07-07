use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::validation::ProofNode;

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

#[derive(Debug, Clone, Serialize, Deserialize, borsh::BorshSerialize, borsh::BorshDeserialize)]
pub struct ScoreStat {
    pub key: u32,
    pub value: i32,
    pub period: i32,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScoresBatchSummary {
    pub fixture_id: u64,
    pub update_stats: ScoresUpdateStats,
    pub event_stats_sub_tree_root: Vec<u8>,
}

#[derive(Debug, Clone, borsh::BorshSerialize, borsh::BorshDeserialize)]
pub struct ScoresBatchSummaryBorsh {
    pub fixture_id: i64,
    pub update_stats: ScoresUpdateStatsBorsh,
    pub events_sub_tree_root: [u8; 32],
}

impl From<&ScoresBatchSummary> for ScoresBatchSummaryBorsh {
    fn from(s: &ScoresBatchSummary) -> Self {
        let mut root = [0u8; 32];
        if s.event_stats_sub_tree_root.len() == 32 {
            root.copy_from_slice(&s.event_stats_sub_tree_root);
        }
        Self {
            fixture_id: s.fixture_id as i64,
            update_stats: ScoresUpdateStatsBorsh::from(&s.update_stats),
            events_sub_tree_root: root,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScoresUpdateStats {
    pub update_count: u64,
    pub min_timestamp: u64,
    pub max_timestamp: u64,
}

#[derive(Debug, Clone, borsh::BorshSerialize, borsh::BorshDeserialize)]
pub struct ScoresUpdateStatsBorsh {
    pub update_count: i32,
    pub min_timestamp: i64,
    pub max_timestamp: i64,
}

impl From<&ScoresUpdateStats> for ScoresUpdateStatsBorsh {
    fn from(u: &ScoresUpdateStats) -> Self {
        Self {
            update_count: u.update_count as i32,
            min_timestamp: u.min_timestamp as i64,
            max_timestamp: u.max_timestamp as i64,
        }
    }
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

#[allow(dead_code)]
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

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SoccerUpdateReference {
    pub id: Option<u64>,
    pub ts: Option<u64>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SoccerPlayerStat {
    pub goals: Option<u32>,
    pub yellow_cards: Option<u32>,
    pub red_cards: Option<u32>,
}
