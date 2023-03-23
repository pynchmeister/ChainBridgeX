use crate::consensus::Consensus;
use crate::crypto::Crypto;
use crate::bridge::Bridge;
use crate::api::handlers::*;
use warp::{self, Filter};

pub fn get_routes(
    consensus: Consensus,
    crypto: Crypto,
    bridge: Bridge,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let consensus_filter = warp::any().map(move || consensus.clone());
    let crypto_filter = warp::any().map(move || crypto.clone());
    let bridge_filter = warp::any().map(move || bridge.clone());

    let transaction_routes = warp::path("transaction")
        .and(warp::post())
        .and(warp::body::json())
        .and(bridge_filter.clone())
        .and_then(add_transaction_handler);

    let balance_routes = warp::path("balance")
        .and(warp::get())
        .and(warp::path::param())
        .and_then(get_balance_handler);

    let transaction_list_routes = warp::path("transactions")
        .and(warp::get())
        .and(bridge_filter.clone())
        .and_then(get_transactions_handler);

    let routes = transaction_routes.or(balance_routes).or(transaction_list_routes);

    routes
}

