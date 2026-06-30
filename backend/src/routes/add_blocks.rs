use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;
use serde_json::json;

use crate::AppState;

#[derive(Deserialize)]
pub struct AddBlockRequest {
    data: String,
}

pub async fn post_block(
    State(state): State<AppState>,
    Json(payload): Json<AddBlockRequest>,
) -> impl IntoResponse {
    // ensure that it's not an empty str
    if payload.data.trim().is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "Data cannot be empty" })),
        )
            .into_response();
    }

    let mut chain = state.lock().unwrap();

    match chain.add_block(payload.data) {
        Ok(()) => (
            StatusCode::CREATED,
            Json(json!({ "message": "Block added" })),
        )
            .into_response(),

        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}
