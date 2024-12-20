use std::str::FromStr as _;
use std::sync::Arc;

use fuels::{crypto::SecretKey, prelude::*};
use kpop;

use crate::args;

#[derive(Clone, Debug)]
pub struct SharedKpop {
    inner: Arc<kpop::Kpop>,
}

impl SharedKpop {
    pub async fn from_args(args: &args::Args) -> Self {
        let provider = Provider::connect(&args.provider_url)
            .await
            .expect("failed to connect");

        let pk =
            SecretKey::from_str(&args.private_key).expect("should be able to parse private key");

        let kp = match &args.contract_id {
            Some(contract_id) => {
                let contract_id = ContractId::from_str(&contract_id)
                    .expect("should be able to parse contract ID");
                kpop::Kpop::load(provider, pk, contract_id)
            }
            None => kpop::Kpop::deploy(provider, pk).await,
        };
        let inner = Arc::new(kp);

        Self { inner }
    }
}
