use axum::{http::Method, routing::get, Router};
use std::sync::{Arc, Mutex};
use tower_http::cors::{Any, CorsLayer};

mod core;
mod routes;
use crate::{
    core::block::Block,
    routes::{add_blocks::post_block, get_blocks::get_blocks},
};
use core::chain::Chain;
use routes::root::root;

pub struct AppState {
    pub chain: Mutex<Chain>,
    pub db: sled::Db,
}

type SharedState = Arc<AppState>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = sled::open("blockchain.db")?;
    let blocks_tree = db.open_tree("blocks")?;

    let mut chain = Chain::new();

    for item in blocks_tree.iter() {
        let (_, block_bytes) = item?;
        let block: Block = serde_json::from_slice(&block_bytes)?;
        chain.blocks.push(block);
    }

    let state: SharedState = Arc::new(AppState {
        chain: Mutex::new(chain),
        db,
    });

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

    axum::serve(listener, app).await.unwrap();

    Ok(())
}
