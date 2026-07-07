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

#[derive(Debug, Clone, borsh::BorshSerialize, borsh::BorshDeserialize)]
pub struct FixtureBorsh {
    pub ts: i64,
    pub start_time: i64,
    pub competition: String,
    pub competition_id: i32,
    pub fixture_group_id: i32,
    pub participant1_id: i32,
    pub participant1: String,
    pub participant2_id: i32,
    pub participant2: String,
    pub fixture_id: i64,
    pub participant1_is_home: bool,
}

impl From<&Fixture> for FixtureBorsh {
    fn from(f: &Fixture) -> Self {
        Self {
            ts: f.ts as i64,
            start_time: f.start_time as i64,
            competition: f.competition.clone(),
            competition_id: f.competition_id as i32,
            fixture_group_id: f.fixture_group_id as i32,
            participant1_id: f.participant1_id as i32,
            participant1: f.participant1.clone(),
            participant2_id: f.participant2_id as i32,
            participant2: f.participant2.clone(),
            fixture_id: f.fixture_id as i64,
            participant1_is_home: f.participant1_is_home,
        }
    }
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

#[derive(Debug, Clone, borsh::BorshSerialize, borsh::BorshDeserialize)]
pub struct FixtureBatchSummaryBorsh {
    pub fixture_id: i64,
    pub competition_id: i32,
    pub competition: String,
    pub update_stats: UpdateStatsBorsh,
    pub update_sub_tree_root: [u8; 32],
}

impl From<&FixtureBatchSummary> for FixtureBatchSummaryBorsh {
    fn from(s: &FixtureBatchSummary) -> Self {
        let mut root = [0u8; 32];
        if s.update_sub_tree_root.len() == 32 {
            root.copy_from_slice(&s.update_sub_tree_root);
        }
        Self {
            fixture_id: s.fixture_id as i64,
            competition_id: s.competition_id as i32,
            competition: s.competition.clone(),
            update_stats: UpdateStatsBorsh::from(&s.update_stats),
            update_sub_tree_root: root,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateStats {
    pub update_count: u64,
    pub min_timestamp: u64,
    pub max_timestamp: u64,
}

#[derive(Debug, Clone, borsh::BorshSerialize, borsh::BorshDeserialize)]
pub struct UpdateStatsBorsh {
    pub update_count: u32,
    pub min_timestamp: i64,
    pub max_timestamp: i64,
}

impl From<&UpdateStats> for UpdateStatsBorsh {
    fn from(u: &UpdateStats) -> Self {
        Self {
            update_count: u.update_count as u32,
            min_timestamp: u.min_timestamp as i64,
            max_timestamp: u.max_timestamp as i64,
        }
    }
}
