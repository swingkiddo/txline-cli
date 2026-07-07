use sha2::{Sha256, Digest};
use crate::types::{Fixture, FixtureBorsh, FixtureBatchSummary, FixtureBatchSummaryBorsh, OddsPayload, OddsPayloadBorsh, OddsBatchSummary, OddsBatchSummaryBorsh, ProofNode, ScoreStat, ScoresBatchSummary, ScoresBatchSummaryBorsh};

pub fn hash_pair(left: &[u8], right: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(left);
    hasher.update(right);
    hasher.finalize().to_vec()
}

pub fn verify_merkle_proof(leaf: &[u8], proof: &[ProofNode], root: &[u8]) -> bool {
    let mut current = leaf.to_vec();
    for node in proof {
        current = if node.is_right_sibling {
            hash_pair(&current, &node.hash)
        } else {
            hash_pair(&node.hash, &current)
        };
    }
    current == root
}

pub fn compute_leaf_hash(data: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}

pub fn hash_fixture(fixture: &Fixture) -> Vec<u8> {
    let borsh_fixture = FixtureBorsh::from(fixture);
    let encoded = borsh::to_vec(&borsh_fixture).expect("fixture borsh serialization");
    compute_leaf_hash(&encoded)
}

pub fn hash_odds(odds: &OddsPayload) -> Vec<u8> {
    let borsh_odds = OddsPayloadBorsh::from(odds);
    let encoded = borsh::to_vec(&borsh_odds).expect("odds borsh serialization");
    compute_leaf_hash(&encoded)
}

pub fn hash_score_stat(stat: &ScoreStat) -> Vec<u8> {
    let encoded = borsh::to_vec(stat).expect("score stat borsh serialization");
    compute_leaf_hash(&encoded)
}

pub fn hash_fixture_summary(summary: &FixtureBatchSummary) -> Vec<u8> {
    let borsh_summary = FixtureBatchSummaryBorsh::from(summary);
    let encoded = borsh::to_vec(&borsh_summary).expect("fixture summary borsh serialization");
    compute_leaf_hash(&encoded)
}

pub fn hash_odds_summary(summary: &OddsBatchSummary) -> Vec<u8> {
    let borsh_summary = OddsBatchSummaryBorsh::from(summary);
    let encoded = borsh::to_vec(&borsh_summary).expect("odds summary borsh serialization");
    compute_leaf_hash(&encoded)
}

pub fn hash_scores_summary(summary: &ScoresBatchSummary) -> Vec<u8> {
    let borsh_summary = ScoresBatchSummaryBorsh::from(summary);
    let encoded = borsh::to_vec(&borsh_summary).expect("scores summary borsh serialization");
    compute_leaf_hash(&encoded)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{ScoresUpdateStats, UpdateStats};
    use bincode::Options;

    fn make_node(hash: Vec<u8>, is_right_sibling: bool) -> ProofNode {
        ProofNode { hash, is_right_sibling }
    }

    fn make_fixture() -> Fixture {
        Fixture {
            fixture_id: 12345,
            start_time: 1700000000,
            participant1: "Arsenal".to_string(),
            participant2: "Chelsea".to_string(),
            participant1_is_home: true,
            competition: "Premier League".to_string(),
            competition_id: 39,
            participant1_id: 100,
            participant2_id: 200,
            fixture_group_id: 5,
            ts: 1700000001,
        }
    }

    fn make_odds() -> OddsPayload {
        OddsPayload {
            fixture_id: 12345,
            message_id: "msg-001".to_string(),
            ts: 1700000002,
            bookmaker: "Bet365".to_string(),
            bookmaker_id: 10,
            super_odds_type: "moneyline".to_string(),
            game_state: Some("PreMatch".to_string()),
            in_running: false,
            market_parameters: None,
            market_period: Some("FullTime".to_string()),
            price_names: vec!["home".to_string(), "draw".to_string(), "away".to_string()],
            prices: vec![1.85, 2.10, 5.50],
            pct: vec!["54.05".to_string(), "47.62".to_string(), "18.18".to_string()],
        }
    }

    fn make_score_stat() -> ScoreStat {
        ScoreStat {
            key: 7,
            value: 2,
            period: 1,
        }
    }

    fn make_fixture_summary() -> FixtureBatchSummary {
        FixtureBatchSummary {
            fixture_id: 12345,
            competition_id: 39,
            competition: "Premier League".to_string(),
            update_stats: UpdateStats {
                update_count: 100,
                min_timestamp: 1699999900,
                max_timestamp: 1700000100,
            },
            update_sub_tree_root: vec![1u8; 32],
        }
    }

    fn make_odds_summary() -> OddsBatchSummary {
        OddsBatchSummary {
            fixture_id: 12345,
            update_stats: UpdateStats {
                update_count: 50,
                min_timestamp: 1699999900,
                max_timestamp: 1700000100,
            },
            odds_sub_tree_root: vec![2u8; 32],
        }
    }

    fn make_scores_summary() -> ScoresBatchSummary {
        ScoresBatchSummary {
            fixture_id: 12345,
            update_stats: ScoresUpdateStats {
                update_count: 25,
                min_timestamp: 1699999900,
                max_timestamp: 1700000100,
            },
            event_stats_sub_tree_root: vec![3u8; 32],
        }
    }

    #[test]
    fn test_hash_pair_deterministic() {
        let a = vec![1, 2, 3];
        let b = vec![4, 5, 6];
        let h1 = hash_pair(&a, &b);
        let h2 = hash_pair(&a, &b);
        assert_eq!(h1, h2);
        assert_eq!(h1.len(), 32);
    }

    #[test]
    fn test_hash_pair_order_matters() {
        let a = vec![1, 2, 3];
        let b = vec![4, 5, 6];
        assert_ne!(hash_pair(&a, &b), hash_pair(&b, &a));
    }

    #[test]
    fn test_verify_empty_proof_leaf_equals_root() {
        let leaf = compute_leaf_hash(b"hello");
        assert!(verify_merkle_proof(&leaf, &[], &leaf));
    }

    #[test]
    fn test_verify_empty_proof_wrong_root() {
        let leaf = compute_leaf_hash(b"hello");
        let wrong_root = compute_leaf_hash(b"world");
        assert!(!verify_merkle_proof(&leaf, &[], &wrong_root));
    }

    #[test]
    fn test_verify_single_node_left_sibling() {
        let leaf = compute_leaf_hash(b"leaf");
        let sibling = compute_leaf_hash(b"sibling");
        let root = hash_pair(&sibling, &leaf);
        let proof = vec![make_node(sibling, false)];
        assert!(verify_merkle_proof(&leaf, &proof, &root));
    }

    #[test]
    fn test_verify_single_node_right_sibling() {
        let leaf = compute_leaf_hash(b"leaf");
        let sibling = compute_leaf_hash(b"sibling");
        let root = hash_pair(&leaf, &sibling);
        let proof = vec![make_node(sibling, true)];
        assert!(verify_merkle_proof(&leaf, &proof, &root));
    }

    #[test]
    fn test_verify_two_level_proof() {
        let leaf = compute_leaf_hash(b"leaf");
        let sib1 = compute_leaf_hash(b"sib1");
        let sib2 = compute_leaf_hash(b"sib2");

        let inner = hash_pair(&sib1, &leaf);
        let root = hash_pair(&inner, &sib2);

        let proof = vec![
            make_node(sib1, false),
            make_node(sib2, true),
        ];
        assert!(verify_merkle_proof(&leaf, &proof, &root));
    }

    #[test]
    fn test_compute_leaf_hash_deterministic() {
        let h1 = compute_leaf_hash(b"test data");
        let h2 = compute_leaf_hash(b"test data");
        assert_eq!(h1, h2);
        assert_eq!(h1.len(), 32);
    }

    #[test]
    fn test_compute_leaf_hash_different_inputs() {
        let h1 = compute_leaf_hash(b"data1");
        let h2 = compute_leaf_hash(b"data2");
        assert_ne!(h1, h2);
    }

    #[test]
    fn test_fixture_borsh_conversion() {
        let f = make_fixture();
        let borsh_f = FixtureBorsh::from(&f);
        assert_eq!(borsh_f.fixture_id, 12345);
        assert_eq!(borsh_f.start_time, 1700000000);
        assert_eq!(borsh_f.competition, "Premier League");
        assert_eq!(borsh_f.competition_id, 39);
        assert_eq!(borsh_f.fixture_group_id, 5);
        assert_eq!(borsh_f.participant1_id, 100);
        assert_eq!(borsh_f.participant1, "Arsenal");
        assert_eq!(borsh_f.participant2_id, 200);
        assert_eq!(borsh_f.participant2, "Chelsea");
        assert!(borsh_f.participant1_is_home);
        assert_eq!(borsh_f.ts, 1700000001);
    }

    #[test]
    fn test_odds_borsh_conversion() {
        let o = make_odds();
        let borsh_o = OddsPayloadBorsh::from(&o);
        assert_eq!(borsh_o.fixture_id, 12345);
        assert_eq!(borsh_o.message_id, "msg-001");
        assert_eq!(borsh_o.ts, 1700000002);
        assert_eq!(borsh_o.bookmaker, "Bet365");
        assert_eq!(borsh_o.bookmaker_id, 10);
        assert_eq!(borsh_o.super_odds_type, "moneyline");
        assert_eq!(borsh_o.game_state.as_deref(), Some("PreMatch"));
        assert!(!borsh_o.in_running);
        assert_eq!(borsh_o.market_parameters, None);
        assert_eq!(borsh_o.market_period.as_deref(), Some("FullTime"));
        assert_eq!(
            borsh_o.price_names,
            vec!["home".to_string(), "draw".to_string(), "away".to_string()]
        );
        assert_eq!(borsh_o.prices, vec![1, 2, 5]);
    }

    #[test]
    fn test_score_stat_borsh_roundtrip() {
        let stat = make_score_stat();
        let encoded = borsh::to_vec(&stat).expect("borsh serialize");
        let decoded: ScoreStat =
            borsh::BorshDeserialize::try_from_slice(&encoded).expect("borsh deserialize");
        assert_eq!(decoded.key, stat.key);
        assert_eq!(decoded.value, stat.value);
        assert_eq!(decoded.period, stat.period);
    }

    #[test]
    fn test_fixture_summary_borsh_conversion() {
        let s = make_fixture_summary();
        let borsh_s = FixtureBatchSummaryBorsh::from(&s);
        assert_eq!(borsh_s.fixture_id, 12345);
        assert_eq!(borsh_s.competition_id, 39);
        assert_eq!(borsh_s.competition, "Premier League");
        assert_eq!(borsh_s.update_stats.update_count, 100);
        assert_eq!(borsh_s.update_stats.min_timestamp, 1699999900);
        assert_eq!(borsh_s.update_stats.max_timestamp, 1700000100);
        assert_eq!(borsh_s.update_sub_tree_root, [1u8; 32]);
    }

    #[test]
    fn test_odds_summary_borsh_conversion() {
        let s = make_odds_summary();
        let borsh_s = OddsBatchSummaryBorsh::from(&s);
        assert_eq!(borsh_s.fixture_id, 12345);
        assert_eq!(borsh_s.update_stats.update_count, 50);
        assert_eq!(borsh_s.update_stats.min_timestamp, 1699999900);
        assert_eq!(borsh_s.update_stats.max_timestamp, 1700000100);
        assert_eq!(borsh_s.odds_sub_tree_root, [2u8; 32]);
    }

    #[test]
    fn test_scores_summary_borsh_conversion() {
        let s = make_scores_summary();
        let borsh_s = ScoresBatchSummaryBorsh::from(&s);
        assert_eq!(borsh_s.fixture_id, 12345);
        assert_eq!(borsh_s.update_stats.update_count, 25);
        assert_eq!(borsh_s.update_stats.min_timestamp, 1699999900);
        assert_eq!(borsh_s.update_stats.max_timestamp, 1700000100);
        assert_eq!(borsh_s.events_sub_tree_root, [3u8; 32]);
    }

    #[test]
    fn test_hash_fixture_deterministic() {
        let f = make_fixture();
        let h1 = hash_fixture(&f);
        let h2 = hash_fixture(&f);
        assert_eq!(h1, h2);
        assert_eq!(h1.len(), 32);
    }

    #[test]
    fn test_hash_odds_deterministic() {
        let o = make_odds();
        let h1 = hash_odds(&o);
        let h2 = hash_odds(&o);
        assert_eq!(h1, h2);
        assert_eq!(h1.len(), 32);
    }

    #[test]
    fn test_hash_score_stat_deterministic() {
        let s = make_score_stat();
        let h1 = hash_score_stat(&s);
        let h2 = hash_score_stat(&s);
        assert_eq!(h1, h2);
        assert_eq!(h1.len(), 32);
    }

    #[test]
    fn test_hash_fixture_summary_deterministic() {
        let s = make_fixture_summary();
        let h1 = hash_fixture_summary(&s);
        let h2 = hash_fixture_summary(&s);
        assert_eq!(h1, h2);
        assert_eq!(h1.len(), 32);
    }

    #[test]
    fn test_hash_odds_summary_deterministic() {
        let s = make_odds_summary();
        let h1 = hash_odds_summary(&s);
        let h2 = hash_odds_summary(&s);
        assert_eq!(h1, h2);
        assert_eq!(h1.len(), 32);
    }

    #[test]
    fn test_hash_scores_summary_deterministic() {
        let s = make_scores_summary();
        let h1 = hash_scores_summary(&s);
        let h2 = hash_scores_summary(&s);
        assert_eq!(h1, h2);
        assert_eq!(h1.len(), 32);
    }

    #[test]
    fn test_borsh_differs_from_bincode() {
        let stat = ScoreStat { key: 1, value: 2, period: 3 };
        let borsh_hash = hash_score_stat(&stat);
        let bincode_bytes = bincode::DefaultOptions::new()
            .serialize(&stat)
            .expect("bincode serialize");
        let bincode_hash = compute_leaf_hash(&bincode_bytes);
        assert_ne!(borsh_hash, bincode_hash);
    }

    #[test]
    fn test_full_merkle_chain_score_stat() {
        let stat = ScoreStat { key: 3, value: 2, period: 4 };
        let leaf = hash_score_stat(&stat);
        assert!(verify_merkle_proof(&leaf, &[], &leaf));
    }
}
