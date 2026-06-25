use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct Block {
    pub index: u64,
    pub data: String,
    pub prev_hash: String,
    pub hash: String,
}
