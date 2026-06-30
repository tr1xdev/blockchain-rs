use axum::{http::Method, routing::get, Router};
use std::sync::{Arc, Mutex};
use tower_http::cors::{Any, CorsLayer};

mod core;
mod routes;
use core::chain::Chain;

use routes::root::root;

use crate::routes::{add_blocks::post_block, get_blocks::get_blocks};

type AppState = Arc<Mutex<Chain>>;

#[tokio::main]
async fn main() {
    let state: AppState = Arc::new(Mutex::new(Chain::new()));

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(Any);

    let app = Router::new()
        .route("/", get(root))
        .route("/blocks", get(get_blocks).post(post_block))
        .layer(cors)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    println!("🚀 Server running on http://localhost:8000");
    println!("   GET  /blocks - get all blocks");
    println!("   POST /blocks - add a block (JSON: {{ \"data\": \"...\" }})");

    axum::serve(listener, app).await.unwrap();
}
