use ethers::prelude::*;
use ethers::abi::Abi;
use std::sync::Arc;
use dotenv::dotenv;
use std::env;
use std::fs;

#[tokio::main]
async fn main() {
    dotenv().ok();

    // Load environment variables
    let rpc_url = env::var("RPC_URL").expect("RPC_URL not set");
    let private_key = env::var("PRIVATE_KEY").expect("PRIVATE_KEY not set");
    let contract_address = env::var("CONTRACT_ADDRESS").expect("CONTRACT_ADDRESS not set");

    // Set up provider and wallet
    let provider = Provider::<Http>::try_from(rpc_url).unwrap();
    let wallet = private_key.parse::<LocalWallet>().unwrap().with_chain_id(31337u64); // Set chain ID
    let client = SignerMiddleware::new(provider, wallet.clone());
    let client = Arc::new(client);

    // Load the ABI from the JSON file
    let abi_json = fs::read_to_string("src/abi.json").expect("Failed to read ABI file");
    let abi: Abi = serde_json::from_str(&abi_json).expect("Failed to parse ABI");

    // Set up contract instance
    let contract_address: Address = contract_address.parse().unwrap();
    let contract = Contract::new(contract_address, abi, client);

    // Stake tokens
    let stake_method = contract.method::<_, H256>("stake", U256::from(100)).unwrap();
    let stake_tx = stake_method.send().await.unwrap();
    println!("Staked 100 tokens. Tx Hash: {:?}", stake_tx);

    // Unstake tokens
    let unstake_method = contract.method::<_, H256>("unstake", U256::from(50)).unwrap();
    let unstake_tx = unstake_method.send().await.unwrap();
    println!("Unstaked 50 tokens. Tx Hash: {:?}", unstake_tx);

    // Check staked balance
    let balance = contract.method::<_, U256>("stakedBalances", wallet.address()).unwrap().call().await.unwrap();
    println!("Staked Balance: {}", balance);
}