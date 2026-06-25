use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use serde::Deserialize;
use std::sync::{Arc, Mutex};

mod core;

use core::block::Block;
use core::chain::Chain;

type AppState = Arc<Mutex<Chain>>;

async fn root() -> &'static str {
    "Hello, World!"
}

async fn get_blocks(State(state): State<AppState>) -> Json<Vec<Block>> {
    let chain = state.lock().unwrap();
    Json(chain.blocks.clone())
}

#[derive(Deserialize)]
struct AddBlockRequest {
    data: String,
}

async fn post_block(
    State(state): State<AppState>,
    Json(payload): Json<AddBlockRequest>,
) -> impl IntoResponse {
    let mut chain = state.lock().unwrap();
    match chain.add_block(payload.data) {
        Ok(()) => (StatusCode::CREATED, "Block added"),
        Err(e) => (StatusCode::BAD_REQUEST, e),
    }
}

#[tokio::main]
async fn main() {
    let state: AppState = Arc::new(Mutex::new(Chain::new()));

    let app = Router::new()
        .route("/", get(root))
        .route("/blocks", get(get_blocks).post(post_block))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    println!("🚀 Server running on http://localhost:8000");
    println!("   GET  /blocks - get all blocks");
    println!("   POST /blocks - add a block (JSON: {{ \"data\": \"...\" }})");

    axum::serve(listener, app).await.unwrap();
}
