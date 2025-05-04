use dotenvy::dotenv;
use ethers::prelude::*;
use ethers::signers::{LocalWallet, Signer};
use futures_util::StreamExt;
use std::env;
use std::str::FromStr;
use std::sync::Arc;

mod bindings;
use bindings::BridgeInitiatedFilter;
use bindings::ZarkToken;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok(); // Load .env file

    // 2. Set everything for L2 A
    let src_ws_url = env::var("SRC_RPC_WS")?;
    let src_contract_address: Address = env::var("SRC_CONTRACT_ADDR")?.parse()?;

    let src_ws = Ws::connect(src_ws_url).await?;
    let src_provider = Provider::new(src_ws);
    let src_client = Arc::new(src_provider.clone());

    let src_private_key = env::var("SRC_PRIVATE_KEY")?;
    let src_wallet: LocalWallet = src_private_key
        .parse::<LocalWallet>()?
        .with_chain_id(31337u64); // same Id, don't mind for now
    let src_signer = Arc::new(SignerMiddleware::new(src_provider, src_wallet.clone()));

    let src_contract = ZarkToken::new(src_contract_address, src_client.clone());

    // 2. Set everything for L2 B
    let dst_ws_url = env::var("DST_RPC_WS")?;
    let dst_contract_address: Address = env::var("DST_CONTRACT_ADDR")?.parse()?;

    let dst_ws = Ws::connect(dst_ws_url).await?;
    let dst_provider = Provider::new(dst_ws);
    let dst_client = Arc::new(dst_provider.clone());

    let dst_private_key = env::var("DST_PRIVATE_KEY")?;
    let dst_wallet: LocalWallet = dst_private_key
        .parse::<LocalWallet>()?
        .with_chain_id(31337u64); // same Id, don't mind for now
    let dst_signer = Arc::new(SignerMiddleware::new(dst_provider, dst_wallet.clone()));

    let dst_contract = ZarkToken::new(dst_contract_address, dst_signer.clone());

    // 3. Set up the event stream
    let binding = src_contract.event::<BridgeInitiatedFilter>();
    let mut stream = binding.stream().await?; // auto-generated type by abigen!

    println!("Listening for BridgeInitiated events from Anvil A 8545...");

    // 5. Loop through the event stream
    while let Some(log) = stream.next().await {
        match log {
            Ok(event) => {
                println!(
                    "Bridging to Anvil A 8555: to = {:?}, amount = {}, srcChain = {}, txHash = {:?}",
                    event.to, event.amount, event.src_chain_id, event.tx_hash
                );

                match dst_contract
                    .mint_from_bridge(event.to, event.amount, event.src_chain_id, event.tx_hash)
                    .send()
                    .await
                {
                    Ok(pending_tx) => {
                        println!("Mint tx sent: {:?}", pending_tx.tx_hash());

                        // Wait for it to be mined and check status
                        match pending_tx.await {
                            Ok(receipt) => {
                                println!("Tx mined: {:?}", receipt);
                                if receipt.unwrap().status == Some(U64::from(1u64)) {
                                    println!("✅ Mint succeeded");
                                } else {
                                    println!("❌ Mint tx failed");
                                }
                            }
                            Err(e) => println!("Error waiting for tx: {:?}", e),
                        }
                    }
                    Err(e) => {
                        eprintln!("Mint failed: {:?}", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("Error reading event: {:?}", e);
            }
        }
    }

    Ok(())
}
