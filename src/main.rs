use std::str::FromStr;

use clap::Parser;
use ethers::prelude::*;
use ethers::utils::parse_units;

#[derive(Parser)]
struct Cli {
    #[arg(short, long, default_value_t = String::from("https://polygon.llamarpc.com"))]
    rpc: String,
    #[arg(short, long)]
    pk: String,
    #[arg(short, long)]
    amount: f32,
    #[arg(short, long)]
    to: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    let explorer = "https://polygonscan.com/tx/";
    let provider = Provider::<Http>::try_from(args.rpc)?;
    let chain_id = provider.get_chainid().await?.as_u64();
    let wallet = args
        .pk
        .parse::<LocalWallet>()
        .unwrap()
        .with_chain_id(chain_id);
    let client = SignerMiddleware::new(provider, wallet);
    let raw_tx = Eip1559TransactionRequest::new()
        .to(Address::from_str(args.to.as_str()).expect("Not a valid address"))
        .value(parse_units(args.amount, "ether")?);

    println!("Sending {} MATIC to {}", args.amount, args.to);
    let tx_receipt = client.send_transaction(raw_tx, None).await?.await?;

    if let Some(r) = tx_receipt {
        if r.status == Some(U64::from(1)) {
            println!("Transaction successful: {}{}", explorer, r.transaction_hash)
        } else {
            println!("Transaction failed: {}{}", explorer, r.transaction_hash)
        }
    } else {
        println!("Transaction not found in the blockchain")
    }

    Ok(())
}
