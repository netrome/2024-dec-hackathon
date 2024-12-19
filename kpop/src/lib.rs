use std::collections::HashMap;

use fuels::{crypto::SecretKey, prelude::*, tx::TxId, types::Bits256};

#[derive(Debug, Clone)]
pub struct Kpop {
    wallet: WalletUnlocked,
    contract_id: ContractId,
}

impl Kpop {
    pub async fn deploy(provider: Provider, pk: SecretKey) -> Self {
        let wallet = WalletUnlocked::new_from_private_key(pk, Some(provider));

        let contract_id = Contract::load_from(
            "../claims-contract/out/debug/claims-contract.bin",
            LoadConfiguration::default(),
        )
        .unwrap()
        .deploy(&wallet, TxPolicies::default())
        .await
        .unwrap()
        .into();

        Self {
            wallet,
            contract_id,
        }
    }

    pub async fn load(_provider: Provider, _pk: SecretKey, _contract_id: ContractId) -> Self {
        todo!();
    }

    pub async fn balance(&self) -> HashMap<String, u64> {
        self.wallet
            .provider()
            .unwrap()
            .get_balances(&self.predicate_address().await.into())
            .await
            .expect("should be able to get balances")
    }

    pub async fn get_claims(&self) -> Vec<claims_contract::Claim> {
        self.contract_instance()
            .await
            .methods()
            .get_claims(&self.wallet.address().into())
            .simulate(Execution::StateReadOnly)
            .await
            .unwrap()
            .value
    }

    pub async fn predicate_address(&self) -> Address {
        self.predicate(self.wallet.address().into())
            .await
            .address()
            .into()
    }

    pub async fn send_to(
        &self,
        address: &Bech32Address,
        asset_id: Option<AssetId>,
        amount: u64,
    ) -> TxId {
        let asset_id =
            asset_id.unwrap_or_else(|| self.wallet.provider().unwrap().base_asset_id().clone());

        let gas = 10_000; // What is sensible?

        let predicate = self.predicate(self.wallet.address().into()).await;
        let input_coins = predicate
            .get_asset_inputs_for_amount(asset_id, amount + gas, None)
            .await
            .expect("should be able to get inputs");
        let output_coin = predicate.get_asset_outputs_for_amount(address, asset_id, amount);

        let mut tb = ScriptTransactionBuilder::prepare_transfer(
            input_coins,
            output_coin,
            TxPolicies::default(),
        );

        tb.add_signer(self.wallet.clone())
            .expect("should be able to add signer");

        let tx = tb
            .build(&self.wallet.provider().unwrap())
            .await
            .expect("should be able to build tx");

        let txid = tx.id(self.wallet.provider().unwrap().chain_id());

        self.wallet
            .provider()
            .unwrap()
            .send_transaction_and_await_commit(tx)
            .await
            .expect("should be able to send transaction");

        txid
    }

    async fn predicate(&self, owner: Address) -> Predicate {
        let predicate_binary_path = "./out/debug/claimable.bin";

        let configurables = claimable_predicate::ClaimableConfigurables::default()
            .with_MAKE_CLAIM_SCRIPT_HASH(
                get_script_bytecode_hash(&self.script_instance(owner).await).await,
            )
            .unwrap()
            .with_OWNER(owner)
            .unwrap();

        Predicate::load_from(predicate_binary_path)
            .unwrap()
            .with_configurables(configurables)
            .with_provider(self.wallet.provider().unwrap().clone())
    }

    async fn script_instance(
        &self,
        owner: Address,
    ) -> make_claim_script::MakeClaim<WalletUnlocked> {
        let script_binary_path = "../make-claim/out/debug/make-claim.bin";

        let configurables = make_claim_script::MakeClaimConfigurables::default()
            .with_CLAIMS_CONTRACT_ADDRESS(Bits256(*self.contract_id))
            .unwrap()
            .with_OWNER(owner)
            .unwrap();

        make_claim_script::MakeClaim::new(self.wallet.clone(), &script_binary_path)
            .with_configurables(configurables)
    }

    async fn contract_instance(&self) -> claims_contract::ClaimsContract<WalletUnlocked> {
        claims_contract::ClaimsContract::new(self.contract_id, self.wallet.clone())
    }
}

async fn get_script_bytecode_hash(
    script_instance: &make_claim_script::MakeClaim<WalletUnlocked>,
) -> Bits256 {
    use sha2::Digest;
    use sha2::Sha256;

    let tx = script_instance
        .main(Address::zeroed(), 0, 0, Bits256::zeroed())
        .build_tx()
        .await
        .unwrap();

    let mut hasher = Sha256::new();
    hasher.update(tx.script());
    let result = hasher.finalize();
    let hash = Bits256(result.try_into().unwrap());
    let h = hex::encode(hash.0);
    println!("Bits: {h}");

    hash
}

mod claimable_predicate {
    use fuels::prelude::*;
    abigen!(Predicate(
        name = "Claimable",
        abi = "../claimable/out/debug/claimable-abi.json"
    ));
}

mod claims_contract {
    use fuels::prelude::*;

    abigen!(Contract(
        name = "ClaimsContract",
        abi = "../claims-contract/out/debug/claims-contract-abi.json"
    ));
}

mod make_claim_script {
    use fuels::prelude::*;

    abigen!(Script(
        name = "MakeClaim",
        abi = "../make-claim/out/debug/make-claim-abi.json",
    ));
}

#[cfg(test)]
mod tests {}