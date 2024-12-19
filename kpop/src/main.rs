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

    //println!("Kpop: {:?}", kp);

    match args.action {
        Action::Predicate => predicate_info(&kp).await,
        Action::Wallet => wallet_info(&kp).await,
        Action::Claims => claims(&kp).await,
        Action::SendTo {
            recipient,
            asset_id,
            amount,
        } => send_to(&kp, recipient, asset_id, amount).await,
        Action::Claim {
            owner,
            asset_id,
            amount,
        } => claim(&kp, owner, asset_id, amount).await,
        Action::Disprove { claim_id } => disprove(&kp, claim_id).await,
        Action::Fulfill { claim_id } => fulfill(&kp, claim_id).await,
    };
}

async fn predicate_info(kp: &kpop::Kpop) {
    let address = kp.predicate_address().await;
    println!("Predicate address: {}", address);
    let balance = kp.predicate_balance().await;
    println!("Balance: {:?}", balance);
}

async fn wallet_info(kp: &kpop::Kpop) {
    println!("Wallet address: {}", kp.wallet.address());
    let balance = kp.wallet_balance().await;
    println!("Balance: {:?}", balance);
}

async fn claims(kp: &kpop::Kpop) {
    let claims = kp.get_claims().await;
    println!("Claims: {:?}", claims);
}

async fn send_to(kp: &kpop::Kpop, recipient: String, asset_id: Option<String>, amount: u64) {
    let recipient = Bech32Address::from_str(&recipient)
        .expect("recipient should be a bech32 formatted address");

    let asset_id =
        asset_id.map(|s| AssetId::from_str(&s).expect("asset_id should be a valid hex string"));

    let txid = kp.send_to(&recipient, asset_id, amount).await;

    println!("Trasaction: {txid}");
}

async fn claim(kp: &kpop::Kpop, owner: String, asset_id: Option<String>, amount: u64) {
    let owner =
        Bech32Address::from_str(&owner).expect("owner should be a bech32 formatted address");

    let asset_id =
        asset_id.map(|s| AssetId::from_str(&s).expect("asset_id should be a valid hex string"));

    let claim_id = kp.claim(owner.into(), asset_id, amount).await;

    println!("Made claim {claim_id}");
}

async fn disprove(kp: &kpop::Kpop, claim_id: u64) {
    kp.disprove_claim(claim_id).await;
    println!("Disproved claim {claim_id}")
}

async fn fulfill(kp: &kpop::Kpop, claim_id: u64) {
    kp.fulfill_claim(claim_id).await;
    println!("Fulfilld claim {claim_id}")
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
    Predicate,
    Wallet,
    Claims,
    SendTo {
        #[arg(long)]
        recipient: String,
        #[arg(long)]
        asset_id: Option<String>,
        #[arg(long)]
        amount: u64,
    },
    Claim {
        #[arg(long)]
        owner: String,
        #[arg(long)]
        asset_id: Option<String>,
        #[arg(long)]
        amount: u64,
    },
    Disprove {
        #[arg(long)]
        claim_id: u64,
    },
    Fulfill {
        #[arg(long)]
        claim_id: u64,
    },
}
