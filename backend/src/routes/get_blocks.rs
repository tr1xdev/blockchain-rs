use axum::{extract::State, Json};

use crate::{core::block::Block, SharedState};

pub async fn get_blocks(State(state): State<SharedState>) -> Json<Vec<Block>> {
    let chain = state.chain.lock().unwrap();
    Json(chain.blocks.clone())
}
