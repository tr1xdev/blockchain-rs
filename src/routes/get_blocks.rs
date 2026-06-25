use axum::{extract::State, Json};

use crate::{core::block::Block, AppState};

pub async fn get_blocks(State(state): State<AppState>) -> Json<Vec<Block>> {
    let chain = state.lock().unwrap();
    Json(chain.blocks.clone())
}
