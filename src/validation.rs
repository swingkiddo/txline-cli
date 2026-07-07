use sha2::{Sha256, Digest};
use crate::types::{Fixture, OddsPayload, ProofNode, ScoreStat};

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
    let encoded = bincode::serialize(fixture).expect("fixture serialization");
    compute_leaf_hash(&encoded)
}

pub fn hash_odds(odds: &OddsPayload) -> Vec<u8> {
    let encoded = bincode::serialize(odds).expect("odds serialization");
    compute_leaf_hash(&encoded)
}

pub fn hash_score_stat(stat: &ScoreStat) -> Vec<u8> {
    let encoded = bincode::serialize(stat).expect("score stat serialization");
    compute_leaf_hash(&encoded)
}
