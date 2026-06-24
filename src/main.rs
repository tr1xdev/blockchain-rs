use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Clone, Serialize)]
struct Block {
    index: u64,
    data: String,
    prev_hash: String,
    hash: String,
}

struct Chain {
    blocks: Vec<Block>,
}

impl Chain {
    fn new() -> Self {
        Self {
            blocks: vec![Self::genesis_block()],
        }
    }

    fn genesis_block() -> Block {
        Block {
            index: 0,
            data: "genesis".to_string(),
            prev_hash: "0".to_string(),
            hash: "genesis".to_string(),
        }
    }

    fn add_block(&mut self, data: String) -> Result<(), &'static str> {
        if data.is_empty() {
            return Err("data cannot be empty");
        }

        let last = self.blocks.last().unwrap();
        let new_index = last.index + 1;
        let hash = format!("{}:{}:{}", new_index, data, last.hash);

        let new_block = Block {
            index: new_index,
            data,
            prev_hash: last.hash.clone(),
            hash,
        };

        let idx = new_block.index;
        self.blocks.push(new_block);
        println!("✅ Block #{} added", idx);
        Ok(())
    }
}

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
    println!("   GET  /blocks  – get all blocks");
    println!("   POST /blocks  – add a block (JSON: {{ \"data\": \"...\" }})");

    axum::serve(listener, app).await.unwrap();
}
