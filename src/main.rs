use anyhow::Result;
use std::{
    str::FromStr, 
    env
};
use web3::types::Address;

mod wallet;
mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    // pub & sec keys
    // let (sec_key, pub_key) = wallet::generate_key_pair();
    // println!("Secret key: {}", &sec_key.to_string());
    // println!("Public key: {}", &pub_key.to_string());

    // pub adress
    // let pub_address = wallet::pub_key_address(&pub_key);
    // println!("Public address: {:?}", pub_address);

    // Display wallet info
    // let wallet = wallet::Wallet::new(&sec_key, &pub_key);
    // println!("Crypto wallet: {:?}", &wallet);

    // // Save Wallet info into the file
    let wallet_path = "wallet.json";
    // wallet.save_info(wallet_path)?;
    
    // // Read Wallet info from the file
    let read_wallet = wallet::Wallet::read_info(wallet_path)?;
    println!("Wallet read successfully: {:}", read_wallet);

    let url = env::var("GOERLI_WS")?;
    let web3_conn = wallet::est_w3_conn(&url).await?;

    let block_num = web3_conn.eth().block_number().await?;
    println!("Block number: {}", &block_num);

    let balance = read_wallet.get_balance_eth(&web3_conn).await?;
    println!("Wallet balance: {} eth", &balance);

    // write any adress that u want that starts from 0x...
    let tx = wallet::create_eth_transaction(Address::from_str("xxxxxxxxx")?, 0.01);
    let tx_hash = wallet::sign_and_send(&web3_conn, tx, &read_wallet.get_sec_key()?).await?;

    println!("Transaction hash: {:?}", tx_hash);

    Ok(())
}
