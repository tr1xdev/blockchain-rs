use axum::{routing::get, Router};
use std::sync::{Arc, Mutex};

mod core;
mod routes;
use core::chain::Chain;

use routes::root::root;

use crate::routes::{add_blocks::post_block, get_blocks::get_blocks};

type AppState = Arc<Mutex<Chain>>;

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
