use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Block {
    pub index: u64,
    pub data: String,
    pub prev_hash: String,
    pub hash: String,
}
