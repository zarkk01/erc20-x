use ethers::prelude::*;
use std::sync::Arc;
use dotenvy::dotenv;
use std::env;
use futures_util::StreamExt;

mod bindings;
use bindings::ZarkToken;
use bindings::BridgeInitiatedFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok(); // Load .env file

    // 1. Load environment variables
    let ws_url = env::var("SRC_RPC_WS")?; // WebSocket URL of source chain
    let contract_address: Address = env::var("SRC_CONTRACT_ADDR")?.parse()?; // Contract address

    // 2. Connect to the WebSocket provider
    let ws = Ws::connect(ws_url).await?;
    let provider = Provider::new(ws);
    let client = Arc::new(provider);

    // 3. Create a ZarkToken contract instance
    let contract = ZarkToken::new(contract_address, client.clone());

    // 4. Set up the event stream
    let filter = contract.event::<BridgeInitiatedFilter>(); // auto-generated type by abigen!
    let mut stream = filter.stream().await?.take(10); // For demo, take 10 events max

    println!("Listening for BridgeInitiated events...");

    // 5. Loop through the event stream
    while let Some(log) = stream.next().await {
        match log {
            Ok(event) => {
                println!(
                    "Event: from = {:?}, to = {:?}, amount = {}, chainId = {}, txHash = {:?}",
                    event.from, event.to, event.amount, event.src_chain_id, event.tx_hash
                );
            }
            Err(e) => {
                eprintln!("Error reading event: {:?}", e);
            }
        }
    }

    Ok(())
}