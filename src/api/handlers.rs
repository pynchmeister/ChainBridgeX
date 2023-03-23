use crate::bridge::{Bridge, BridgeConfig};
use crate::consensus::{Consensus, ConsensusConfig};
use crate::crypto::{Crypto, CryptoConfig};
use crate::database::{Database, DatabaseConfig};
use crate::utils::ConfigError;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use warp::{self, Filter};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuccessResponse {
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddTransactionRequest {
    pub from_address: String,
    pub to_address: String,
    pub amount: u64,
}

pub async fn add_transaction_handler(
    request: AddTransactionRequest,
    bridge: Arc<Bridge>,
) -> Result<impl warp::Reply, warp::Rejection> {
    bridge.add_transaction(request.from_address, request.to_address, request.amount);

    let response = SuccessResponse {
        message: "Transaction added to the mempool".to_owned(),
    };

    Ok(warp::reply::json(&response))
}

pub async fn get_balance_handler(
    address: String,
    blockchain: Arc<RwLock<Blockchain>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let balance = blockchain.read().await.get_balance(&address);

    let response = SuccessResponse {
        message: format!("Balance for address {} is {}", address, balance),
    };

    Ok(warp::reply::json(&response))
}

pub async fn get_transactions_handler(
    blockchain: Arc<RwLock<Blockchain>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let transactions = blockchain.read().await.get_transactions();

    Ok(warp::reply::json(&transactions))
}

pub fn get_routes(
    consensus: Consensus,
    crypto: Crypto,
    bridge: Bridge,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let blockchain = Arc::new(RwLock::new(Blockchain::new(bridge.get_database())));

    let consensus_filter = warp::any().map(move || consensus.clone());
    let crypto_filter = warp::any().map(move || crypto.clone());
    let bridge_filter = warp::any().map(move || bridge.clone());
    let blockchain_filter = warp::any().map(move || blockchain.clone());

    let add_transaction = warp::path!("transaction")
        .and(warp::post())
        .and(warp::body::json())
        .and(bridge_filter.clone())
        .and_then(add_transaction_handler);

    let get_balance = warp::path!("balance" / String)
        .and(warp::get())
        .and(blockchain_filter.clone())
        .and_then(get_balance_handler);

    let get_transactions = warp::path!("transactions")
        .and(warp::get())
        .and(blockchain_filter.clone())
        .and_then(get_transactions_handler);

    let routes = add_transaction.or(get_balance).or(get_transactions);

    routes
}

