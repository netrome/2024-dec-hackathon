use std::sync::Arc;
use std::{collections::HashMap, str::FromStr as _};

use fuels::{crypto::SecretKey, prelude::*};
use kpop;

use crate::args;
use crate::model;

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

impl SharedKpop {
    pub async fn wallet_balance(&self) -> HashMap<String, u64> {
        self.inner.wallet_balance().await
    }

    pub async fn predicate_balance(&self) -> HashMap<String, u64> {
        self.inner.predicate_balance().await
    }

    pub async fn get_claims(&self) -> Vec<model::Claim> {
        self.inner
            .get_claims()
            .await
            .into_iter()
            .map(model::Claim::from)
            .collect()
    }

    pub async fn predicate_address(&self) -> String {
        self.inner.predicate_address().await.to_string()
    }

    pub async fn fund_predicate(&self, asset_id: Option<AssetId>, amount: u64) -> String {
        self.inner
            .fund_predicate(asset_id, amount)
            .await
            .to_string()
    }

    pub async fn send_to(
        &self,
        address: &Bech32Address,
        asset_id: Option<AssetId>,
        amount: u64,
    ) -> String {
        self.inner
            .send_to(address, asset_id, amount)
            .await
            .to_string()
    }

    pub async fn disprove_claim(&self, claim_id: u64) {
        self.inner.disprove_claim(claim_id).await
    }

    pub async fn fulfill_claim(&self, claim_id: u64) {
        self.inner.fulfill_claim(claim_id).await
    }

    pub async fn claim(&self, owner: Address, asset_id: Option<AssetId>, amount: u64) -> u64 {
        self.inner.claim(owner, asset_id, amount).await
    }
}

impl From<kpop::claims_contract::Claim> for model::Claim {
    fn from(value: kpop::claims_contract::Claim) -> Self {
        let amount = value.amount;
        let claim_id = value.id;
        let owner = value.owner.to_string();
        let recipient = value.recipient.to_string();
        let asset_id = value.asset.to_string();
        let block_height = value.block_height;

        Self {
            amount,
            claim_id,
            owner,
            recipient,
            block_height,
            asset_id,
        }
    }
}
