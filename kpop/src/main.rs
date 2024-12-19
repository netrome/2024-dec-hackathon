use std::str::FromStr;

use clap::Parser;
use clap::Subcommand;
use fuels::{crypto::SecretKey, prelude::*};

use kpop;

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let provider = Provider::connect(&args.provider_url)
        .await
        .expect("failed to connect");

    let pk = SecretKey::from_str(&args.private_key).expect("should be able to parse private key");

    let kp = match args.contract_id {
        Some(contract_id) => {
            let contract_id =
                ContractId::from_str(&contract_id).expect("should be able to parse contract ID");
            kpop::Kpop::load(provider, pk, contract_id)
        }
        None => kpop::Kpop::deploy(provider, pk).await,
    };

    println!("Kpop: {:?}", kp);

    match args.action {
        Action::Balance => balance(&kp).await,
        Action::Predicate => predicate_address(&kp).await,
        Action::Wallet => wallet_address(&kp),
        Action::SendTo {
            recipient,
            asset_id,
            amount,
        } => send_to(&kp, recipient, asset_id, amount).await,
    };
}

async fn balance(kp: &kpop::Kpop) {
    let balance = kp.balance().await;
    println!("Balance: {:?}", balance);
}

async fn predicate_address(kp: &kpop::Kpop) {
    let address = kp.predicate_address().await;
    println!("Predicate address: {}", address);
}

fn wallet_address(kp: &kpop::Kpop) {
    println!("Wallet address: {}", kp.wallet.address())
}

async fn send_to(kp: &kpop::Kpop, recipient: String, asset_id: Option<String>, amount: u64) {
    let recipient = Bech32Address::from_str(&recipient)
        .expect("recipient should be a bech32 formatted address");

    let asset_id =
        asset_id.map(|s| AssetId::from_str(&s).expect("asset_id should be a valid hex string"));

    let txid = kp.send_to(&recipient, asset_id, amount).await;

    println!("Trasaction: {txid}");
}

#[derive(Parser)]
struct Args {
    /// URL of Fuel node to connect to
    #[arg(long, env)]
    provider_url: String,

    /// Private key - shh, don't tell anyone!!!
    #[arg(long, env)]
    private_key: String,

    /// ID of claims contract. Will deploy a new one if not provided.
    #[arg(long, env)]
    contract_id: Option<String>,

    #[command(subcommand)]
    action: Action,
}

#[derive(Subcommand)]
enum Action {
    Balance,
    Predicate,
    Wallet,
    SendTo {
        #[arg(long)]
        recipient: String,
        #[arg(long)]
        asset_id: Option<String>,
        #[arg(long)]
        amount: u64,
    },
}
