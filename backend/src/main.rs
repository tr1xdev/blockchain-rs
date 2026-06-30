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

    println!("Loading blocks from database...");
    let mut chain = Chain::new();

    let items: Vec<(sled::IVec, sled::IVec)> = blocks_tree.iter().collect::<Result<Vec<_>, _>>()?;

    let mut parsed_items: Vec<(Vec<u8>, Vec<u8>)> = items
        .into_iter()
        .map(|(k, v)| (k.to_vec(), v.to_vec()))
        .collect();

    parsed_items.sort_by(|a, b| a.0.cmp(&b.0));

    for (_, block_bytes) in parsed_items {
        let block: Block = serde_json::from_slice(&block_bytes)?;
        chain.blocks.push(block);
    }

    println!(
        "Successfully loaded {} blocks from database.",
        chain.blocks.len()
    );

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
    println!("   GET  /blocks - get all blocks");
    println!("   POST /blocks - add a block (JSON: {{ \"data\": \"...\" }})");

    axum::serve(listener, app).await.unwrap();

    Ok(())
}
