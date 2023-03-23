use crate::blockchain::{Blockchain, SharedBlockchain};
use crate::pow::ProofOfWork;
use crate::transaction::Transaction;
use std::sync::{Arc, Mutex};
use warp::{self, http::StatusCode, reply::json, Reply};

pub async fn get_block(
    index: u32,
    blockchain: SharedBlockchain,
) -> Result<impl warp::Reply, warp::Rejection> {
    let blockchain = blockchain.lock().unwrap();
    match blockchain.blocks.get(index as usize) {
        Some(block) => Ok(json(block)),
        None => Err(warp::reject::not_found()),
    }
}

pub async fn get_blocks(
    blockchain: SharedBlockchain,
) -> Result<impl warp::Reply, warp::Rejection> {
    let blockchain = blockchain.lock().unwrap();
    Ok(json(&blockchain.blocks))
}

pub async fn mine_block(
    blockchain: SharedBlockchain,
    pow: ProofOfWork,
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut blockchain = blockchain.lock().unwrap();

    let reward_transaction = Transaction::new_coinbase();
    blockchain.add_transaction(reward_transaction);

    let mut block = match blockchain.mine_block() {
        Some(block) => block,
        None => return Err(warp::reject::bad_request()),
    };

    if !pow.validate(&block) {
        return Err(warp::reject::bad_request());
    }

    Ok(json(&block))
}

pub async fn add_transaction(
    transaction: Transaction,
    blockchain: SharedBlockchain,
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut blockchain = blockchain.lock().unwrap();
    if blockchain.add_transaction(transaction) {
        Ok(StatusCode::CREATED)
    } else {
        Err(warp::reject::bad_request())
    }
}

