use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProofNode {
    pub hash: Vec<u8>,
    pub is_right_sibling: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult<T: Serialize> {
    pub data: T,
    pub sub_tree_valid: bool,
    pub main_tree_valid: bool,
}
