use chainbridgex::blockchain::Blockchain;
use chainbridgex::handlers::{get_block, get_blocks, mine_block};
use chainbridgex::pow::ProofOfWork;
use chainbridgex::utils::generate_uuid;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use warp::{self, Filter, Reply};

type SharedBlockchain = Arc<Mutex<Blockchain>>;

#[tokio::main]
async fn main() {
    let blockchain = Arc::new(Mutex::new(Blockchain::new()));
    let pow = ProofOfWork::new(4);

    let get_block_route = warp::path!("blocks" / u32)
        .and(warp::get())
        .and(with_blockchain(blockchain.clone()))
        .and_then(get_block);

    let get_blocks_route = warp::path!("blocks")
        .and(warp::get())
        .and(with_blockchain(blockchain.clone()))
        .and_then(get_blocks);

    let mine_block_route = warp::path!("mine")
        .and(warp::post())
        .and(with_blockchain(blockchain.clone()))
        .and(with_pow(pow))
        .and_then(mine_block);

    let routes = get_block_route.or(get_blocks_route).or(mine_block_route);

    println!("Starting server at 127.0.0.1:3030");
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

fn with_blockchain(
    blockchain: SharedBlockchain,
) -> impl Filter<Extract = (SharedBlockchain,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || blockchain.clone())
}

fn with_pow(
    pow: ProofOfWork,
) -> impl Filter<Extract = (ProofOfWork,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || pow.clone())
}

