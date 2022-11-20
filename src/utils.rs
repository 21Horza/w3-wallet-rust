#![allow(dead_code)]
use std::time::{SystemTime, UNIX_EPOCH};
use web3::types::U256;

// convert system time to a num of seconds since the epoch 
// until the time passed into the function
pub fn get_num_sec_time() -> u64 {
    let dur = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    dur.as_secs() << 30 | dur.subsec_nanos() as u64
}

pub fn wei_to_eth(wei_val: U256) -> f64 {
    let result = wei_val.as_u128() as f64;
    result / 1_000_000_000_000_000_000.0
}

pub fn eth_to_wei(eth: f64) -> U256 {
    let result = eth * 1_000_000_000_000_000_000.0;
    let result = result as u128;

    U256::from(result)
}