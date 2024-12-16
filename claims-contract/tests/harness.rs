use fuels::{prelude::*, types::ContractId};

// Load abi from json
abigen!(Contract(
    name = "MyContract",
    abi = "out/debug/claims-contract-abi.json"
));

async fn get_contract_instance() -> (MyContract<WalletUnlocked>, ContractId, Vec<WalletUnlocked>) {
    // Launch a local network and deploy the contract
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
async fn can_get_contract_id() {
    let (_instance, _id, _wallets) = get_contract_instance().await;

    // Now you have an instance of your contract you can use to test each function
}

#[tokio::test]
async fn can_initiate_claim() {
    let (instance, _id, mut wallets) = get_contract_instance().await;

    let owner = wallets.pop().unwrap();
    let recipient = wallets.pop().unwrap();

    instance
        .clone()
        .with_account(owner.clone())
        .methods()
        .initiate_claim(owner.address(), recipient.address())
        .call()
        .await
        .unwrap();

    let claims = instance
        .methods()
        .get_claims(owner.address())
        .call()
        .await
        .unwrap();

    println!("Claims: {claims:?}");
    assert!(claims.value.len() == 1);

    // Now you have an instance of your contract you can use to test each function
}
