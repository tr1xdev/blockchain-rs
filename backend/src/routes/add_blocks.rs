use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;

use crate::AppState;

#[derive(Deserialize)]
pub struct AddBlockRequest {
    data: String,
}

pub async fn post_block(
    State(state): State<AppState>,
    Json(payload): Json<AddBlockRequest>,
) -> impl IntoResponse {
    let mut chain = state.lock().unwrap();
    match chain.add_block(payload.data) {
        Ok(()) => (StatusCode::CREATED, "Block added"),
        Err(e) => (StatusCode::BAD_REQUEST, e),
    }
}
