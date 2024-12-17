use fuels::{
    prelude::*,
    types::{errors::transaction::Reason, ContractId},
};

abigen!(Contract(
    name = "MyContract",
    abi = "out/debug/claims-contract-abi.json"
));

async fn get_contract_instance() -> (MyContract<WalletUnlocked>, ContractId, Vec<WalletUnlocked>) {
    let mut wallets = launch_custom_provider_and_get_wallets(
        WalletsConfig::new(
            Some(3),             /* Single wallet */
            Some(1),             /* Single coin (UTXO) */
            Some(1_000_000_000), /* Amount per coin */
        ),
        None,
        None,
    )
    .await
    .unwrap();
    let wallet = wallets.pop().unwrap();

    let id = Contract::load_from(
        "./out/debug/claims-contract.bin",
        LoadConfiguration::default(),
    )
    .unwrap()
    .deploy(&wallet, TxPolicies::default())
    .await
    .unwrap();

    let instance = MyContract::new(id.clone(), wallet);

    (instance, id.into(), wallets)
}

#[tokio::test]
async fn can_initiate_claim() {
    let (instance, _id, mut wallets) = get_contract_instance().await;

    let owner = wallets.pop().unwrap();
    let recipient = wallets.pop().unwrap();
    let amount = 10_000;

    let call_params = CallParameters::default().with_amount(amount);

    instance
        .clone()
        .with_account(owner.clone())
        .methods()
        .initiate_claim(owner.address(), recipient.address())
        .call_params(call_params)
        .unwrap()
        .call()
        .await
        .unwrap();

    let claims = instance
        .methods()
        .get_claims(owner.address())
        .call()
        .await
        .unwrap();

    let contract_balance = *instance
        .get_balances()
        .await
        .unwrap()
        .get(&AssetId::zeroed())
        .unwrap();
    assert_eq!(contract_balance, amount);

    assert!(claims.value.len() == 1);
    assert_eq!(claims.value.get(0).unwrap().owner, owner.address().into());
    assert_eq!(claims.value.get(0).unwrap().amount, amount);
}

#[tokio::test]
async fn can_disprove_claim() {
    let (instance, _id, mut wallets) = get_contract_instance().await;

    let owner = wallets.pop().unwrap();
    let recipient = wallets.pop().unwrap();

    let call_params = CallParameters::default().with_amount(10_000);

    let claim_id = instance
        .clone()
        .with_account(owner.clone())
        .methods()
        .initiate_claim(owner.address(), recipient.address())
        .call_params(call_params)
        .unwrap()
        .call()
        .await
        .unwrap()
        .value;

    instance
        .clone()
        .with_account(owner)
        .methods()
        .disprove(claim_id)
        .with_variable_output_policy(VariableOutputPolicy::Exactly(1))
        .call()
        .await
        .expect("should be able to disprove");

    let contract_balance = *instance
        .get_balances()
        .await
        .unwrap()
        .get(&AssetId::zeroed())
        .unwrap();

    assert_eq!(contract_balance, 0);
}

#[tokio::test]
async fn cant_fulfill_claim_before_min_height_reached() {
    let (instance, _id, mut wallets) = get_contract_instance().await;

    let owner = wallets.pop().unwrap();
    let recipient = wallets.pop().unwrap();

    let call_params = CallParameters::default().with_amount(10_000);

    let claim_id = instance
        .clone()
        .with_account(owner.clone())
        .methods()
        .initiate_claim(owner.address(), recipient.address())
        .call_params(call_params)
        .unwrap()
        .call()
        .await
        .unwrap()
        .value;

    let res = instance
        .with_account(recipient)
        .methods()
        .fulfill(claim_id)
        .with_variable_output_policy(VariableOutputPolicy::Exactly(1))
        .call()
        .await;

    let Error::Transaction(Reason::Reverted { reason, .. }) = res.unwrap_err() else {
        panic!("expected reverted transaction")
    };

    assert_eq!(reason, "TooSoon(112)");
}

#[tokio::test]
async fn can_fulfill_claim_after_min_height_reached() {
    let (instance, _id, mut wallets) = get_contract_instance().await;

    let owner = wallets.pop().unwrap();
    let recipient = wallets.pop().unwrap();

    let call_params = CallParameters::default().with_amount(10_000);

    let claim_id = instance
        .clone()
        .with_account(owner.clone())
        .methods()
        .initiate_claim(owner.address(), recipient.address())
        .call_params(call_params)
        .unwrap()
        .call()
        .await
        .unwrap()
        .value;

    owner
        .provider()
        .unwrap()
        .produce_blocks(112, None)
        .await
        .expect("should be able to produce blocks");

    instance
        .clone()
        .with_account(recipient)
        .methods()
        .fulfill(claim_id)
        .with_variable_output_policy(VariableOutputPolicy::Exactly(1))
        .call()
        .await
        .expect("should be able to fulfill");

    let contract_balance = *instance
        .get_balances()
        .await
        .unwrap()
        .get(&AssetId::zeroed())
        .unwrap();

    assert_eq!(contract_balance, 0);
}
