use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProofNode {
    pub hash: Vec<u8>,
    pub is_right_sibling: bool,
}
