#![allow(dead_code)]

use crate::utils;

// key pars generation
use secp256k1::{
    rand::rngs,
    PublicKey, key::SecretKey,
};

// connect to Rinkeby, generate addr, get balance
use web3::{
    transports::WebSocket,
    transports,
    types::{Address, TransactionParameters, H256, U256},
    Web3,
};

// Wallet Struct essentials
use anyhow::Result;
use std::{
    fmt, 
    str::FromStr, 
    fs::OpenOptions, 
    io::{BufWriter, BufReader}
};
use serde::{Deserialize, Serialize};

// hash key address
use tiny_keccak::keccak256;

pub fn generate_key_pair() -> (SecretKey, PublicKey) {
    let secp = secp256k1::Secp256k1::new();
    let mut rng = rngs::JitterRng::new_with_timer(utils::get_num_sec_time);
    secp.generate_keypair(&mut rng)
}

pub fn pub_key_address(pub_key: &PublicKey) -> Address {
    let pub_key = pub_key.serialize_uncompressed();

    debug_assert_eq!(pub_key[0], 0x04);
    let hash = keccak256(&pub_key[1..]);

    Address::from_slice(&hash[12..])
}

pub async fn est_w3_conn(url: &str) -> Result<Web3<WebSocket>> {
    let transport = web3::transports::WebSocket::new(url).await?;
    Ok(web3::Web3::new(transport))
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Wallet {
    pub sec_key: String,
    pub pub_key: String,
    pub pub_address: String,
}

impl fmt::Display for Wallet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s_key = &self.sec_key;
        let p_key = &self.pub_key;
        let addr = &self.pub_address;
        write!(f, " Secret key: {} , ", s_key);
        write!(f, " Public key: {} , ", p_key);
        write!(f, " Public adress: {}", addr)
    }
}

impl Wallet {
    pub fn new(sec_key: &SecretKey, pub_key: &PublicKey) -> Self {
        let address: Address = pub_key_address(&pub_key);
        Wallet {
            sec_key: sec_key.to_string(),
            pub_key: pub_key.to_string(),
            pub_address: format!("{:?}", address)
        }
    }
    
    pub fn save_info(&self, path: &str) -> Result<()> {
        let file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(path)?;
        
        let buf_writer = BufWriter::new(file);
        
        serde_json::to_writer_pretty(buf_writer, self)?;
        
        Ok(())
    }
    
    pub fn read_info(path: &str) -> Result<Wallet> {
        let file = OpenOptions::new().read(true).open(path)?;
        let buf_reader = BufReader::new(file);
        
        let wallet: Wallet = serde_json::from_reader(buf_reader)?;
        Ok(wallet)
    }
    
    pub fn get_sec_key(&self) -> Result<SecretKey> {
        let sec_key = SecretKey::from_str(&self.sec_key)?;
        Ok(sec_key)
    }
    
    pub fn get_pub_key(&self) -> Result<PublicKey> {
        let pub_key = PublicKey::from_str(&self.pub_key)?;
        Ok(pub_key)
    }
    
    pub async fn get_balance(&self, w3_conn: &Web3<WebSocket>) -> Result<U256> {
        let wallet_addr = Address::from_str(&self.pub_address)?;
        
        let balance = w3_conn.eth().balance(wallet_addr, None).await?;
        
        Ok(balance)
    }
    
    pub async fn get_balance_eth(&self, w3_conn: &Web3<web3::transports::WebSocket>) -> Result<f64> {
        let wei_balance = self.get_balance(w3_conn).await?;
        Ok(utils::wei_to_eth(wei_balance))
    }
    
}

pub fn create_eth_transaction(to: Address, eth: f64) -> TransactionParameters {
    TransactionParameters {
        to: Some(to),
        value: utils::eth_to_wei(eth),
        ..Default::default()
    }
}

pub async fn sign_and_send(
    web3: &Web3<transports::WebSocket>,
    transaction: TransactionParameters,
    secret_key: &secp256k1::key::SecretKey,
) -> Result<H256> {
    let signed = web3
        .accounts()
        .sign_transaction(transaction, secret_key)
        .await?;

    let transaction_result = web3
        .eth()
        .send_raw_transaction(signed.raw_transaction)
        .await?;
    Ok(transaction_result)
}