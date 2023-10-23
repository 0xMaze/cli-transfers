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
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    let provider = Provider::<Http>::try_from(args.rpc)?;
    let chain_id = provider.get_chainid().await?.as_u64();
    let wallet: LocalWallet = args
        .pk
        .parse::<LocalWallet>()
        .unwrap()
        .with_chain_id(chain_id);
    let client = SignerMiddleware::new(provider, wallet);
    let raw_tx = TransactionRequest::pay(client.address(), parse_units(args.amount, "ether")?);
    let tx = client.send_transaction(raw_tx, None).await?;

    println!("{:#?}", tx);

    Ok(())
}
