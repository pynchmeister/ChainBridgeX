pub mod api;
pub mod bridge;
pub mod consensus;
pub mod crypto;
pub mod database;
pub mod execution;
pub mod utils;

use std::sync::Arc;
use tokio::sync::RwLock;
use warp::{self, Filter};

#[tokio::main]
async fn main() {
    let bridge_config = BridgeConfig::default();
    let consensus_config = ConsensusConfig::default();
    let database_config = DatabaseConfig::default();
    let crypto_config = CryptoConfig::default();

    let db = database::initialize_database(&database_config).await.unwrap();
    let blockchain = Arc::new(RwLock::new(Blockchain::new(db.clone())));

    let consensus = consensus::initialize_consensus(&consensus_config, blockchain.clone()).await;
    let consensus_filter = warp::any().map(move || consensus.clone());

    let crypto = crypto::initialize_crypto(&crypto_config);
    let crypto_filter = warp::any().map(move || crypto.clone());

    let bridge = bridge::initialize_bridge(&bridge_config, blockchain.clone(), consensus.clone(), crypto.clone());
    let bridge_filter = warp::any().map(move || bridge.clone());

    let routes = api::routes::get_routes()
        .with(consensus_filter)
        .with(crypto_filter)
        .with(bridge_filter);

    println!("Starting server at 127.0.0.1:3030");
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

