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

    fn make_node(hash: Vec<u8>, is_right_sibling: bool) -> ProofNode {
        ProofNode { hash, is_right_sibling }
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
}
