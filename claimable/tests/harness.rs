use std::{fmt::Debug, hash::Hash};

use fuels::{
    accounts::{predicate::Predicate, wallet::WalletUnlocked, Account},
    client::{FuelClient, PaginationRequest},
    crypto::SecretKey,
    prelude::*,
    types::{
        transaction_builders::{BuildableTransaction, ScriptTransactionBuilder},
        Bits256, Bytes32,
    },
};

use fuel_core_client::client::types::TransactionStatus;

mod claimable_predicate {
    use fuels::prelude::*;
    abigen!(Predicate(
        name = "Claimable",
        abi = "out/debug/claimable-abi.json"
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

struct Harness {
    wallet_0: WalletUnlocked,
    wallet_1: WalletUnlocked,
    wallet_2: WalletUnlocked,
    provider: Provider,
    asset_id: AssetId,
    contract_instance: claims_contract::ClaimsContract<WalletUnlocked>,
    script_instance: make_claim_script::MakeClaim<WalletUnlocked>,
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

async fn setup_wallets_and_network() -> Harness {
    // WALLETS
    let private_key_0: SecretKey =
        "0xc2620849458064e8f1eb2bc4c459f473695b443ac3134c82ddd4fd992bd138fd"
            .parse()
            .unwrap();
    let private_key_1: SecretKey =
        "0x37fa81c84ccd547c30c176b118d5cb892bdb113e8e80141f266519422ef9eefd"
            .parse()
            .unwrap();
    let private_key_2: SecretKey =
        "0x976e5c3fa620092c718d852ca703b6da9e3075b9f2ecb8ed42d9f746bf26aafb"
            .parse()
            .unwrap();

    let mut wallet_0: WalletUnlocked = WalletUnlocked::new_from_private_key(private_key_0, None);
    let mut wallet_1: WalletUnlocked = WalletUnlocked::new_from_private_key(private_key_1, None);
    let mut wallet_2: WalletUnlocked = WalletUnlocked::new_from_private_key(private_key_2, None);

    // TOKENS
    let asset_id = AssetId::default();

    let all_coins = [&wallet_0, &wallet_1, &wallet_2]
        .iter()
        .flat_map(|wallet| {
            setup_single_asset_coins(wallet.address(), AssetId::default(), 10, 1_000_000)
        })
        .collect::<Vec<_>>();

    // NETWORKS
    let node_config = NodeConfig::default();

    let provider = setup_test_provider(all_coins, vec![], Some(node_config), None)
        .await
        .unwrap();

    [&mut wallet_0, &mut wallet_1, &mut wallet_2]
        .iter_mut()
        .for_each(|wallet| {
            wallet.set_provider(provider.clone());
        });

    // CONTRACT
    let id = Contract::load_from(
        "../claims-contract/out/debug/claims-contract.bin",
        LoadConfiguration::default(),
    )
    .unwrap()
    .deploy(&wallet_0, TxPolicies::default())
    .await
    .unwrap();

    let contract_instance = claims_contract::ClaimsContract::new(id, wallet_0.clone());

    // SCRIPT
    let script_binary_path = "../make-claim/out/debug/make-claim.bin";
    let script_instance = make_claim_script::MakeClaim::new(wallet_1.clone(), &script_binary_path);

    let configurables = make_claim_script::MakeClaimConfigurables::default()
        .with_CLAIMS_CONTRACT_ADDRESS(Bits256(*contract_instance.contract_id().hash))
        .unwrap()
        .with_OWNER(wallet_0.address().into())
        .unwrap();

    let script_instance = script_instance.with_configurables(configurables);

    return Harness {
        wallet_0,
        wallet_1,
        wallet_2,
        provider,
        asset_id,
        contract_instance,
        script_instance,
    };
}

async fn get_last_tx_fee(client: &FuelClient) -> u64 {
    let status = client
        .transactions(PaginationRequest {
            cursor: None,
            results: 1,
            direction: fuels::client::PageDirection::Forward,
        })
        .await
        .unwrap()
        .results[0]
        .status
        .clone();

    if let TransactionStatus::Success { total_fee, .. } = status {
        total_fee
    } else {
        0
    }
}

#[tokio::test]
async fn owner_can_spend_claimable_predicate() -> Result<()> {
    let harness = setup_wallets_and_network().await;
    let client = FuelClient::new(harness.provider.url()).unwrap();

    // CONFIGURABLES
    let owner_wallet = harness.wallet_0;
    let owner_address: Address = owner_wallet.address().into();

    let configurables = claimable_predicate::ClaimableConfigurables::default()
        .with_MAKE_CLAIM_SCRIPT_HASH(Bits256::zeroed())?
        .with_OWNER(owner_address)?;

    // PREDICATE
    let predicate_binary_path = "./out/debug/claimable.bin";
    let predicate: Predicate = Predicate::load_from(predicate_binary_path)?
        .with_provider(harness.provider.clone())
        .with_configurables(configurables);

    // FUND PREDICATE
    let claimable_amount = 100;
    let wallet_0_amount = harness
        .provider
        .get_asset_balance(owner_wallet.address(), harness.asset_id)
        .await?;

    owner_wallet
        .transfer(
            predicate.address(),
            claimable_amount,
            harness.asset_id,
            TxPolicies::default(),
        )
        .await?;
    let mut accumulated_fee = get_last_tx_fee(&client).await;

    // BUILD TRANSACTION
    let mut tb: ScriptTransactionBuilder = {
        let input_coin = predicate
            .get_asset_inputs_for_amount(harness.asset_id, 1, None)
            .await?;
        let output_coin = predicate.get_asset_outputs_for_amount(
            owner_wallet.address(),
            harness.asset_id,
            claimable_amount - 1,
        ); // minus 1 for gas

        ScriptTransactionBuilder::prepare_transfer(input_coin, output_coin, TxPolicies::default())
    };

    // SIGN TRANSACTION
    tb.add_signer(owner_wallet.clone())?;

    assert_eq!(
        harness
            .provider
            .get_asset_balance(predicate.address(), harness.asset_id)
            .await?,
        claimable_amount
    );
    assert_eq!(
        harness
            .provider
            .get_asset_balance(owner_wallet.address(), harness.asset_id)
            .await?,
        wallet_0_amount - claimable_amount - accumulated_fee
    );

    // SPEND PREDICATE
    let tx: ScriptTransaction = tb.build(harness.provider.clone()).await?;
    harness
        .provider
        .send_transaction_and_await_commit(tx)
        .await?;
    accumulated_fee += get_last_tx_fee(&client).await;

    assert_eq!(
        harness
            .provider
            .get_asset_balance(predicate.address(), harness.asset_id)
            .await?,
        0
    );
    assert_eq!(
        harness
            .provider
            .get_asset_balance(owner_wallet.address(), harness.asset_id)
            .await?,
        wallet_0_amount - accumulated_fee
    );

    Ok(())
}

#[tokio::test]
async fn recipient_can_initiate_a_claim_from_a_claimable_predicate() -> Result<()> {
    let harness = setup_wallets_and_network().await;
    let client = FuelClient::new(harness.provider.url()).unwrap();

    // CONFIGURABLES
    let owner_wallet = harness.wallet_0;
    let owner_address: Address = owner_wallet.address().into();

    let recipient_wallet = harness.wallet_1;
    let recipient_address: Address = recipient_wallet.address().into();

    let script_hash_hex = include_str!("../../make-claim/out/debug/make-claim-bin-hash");
    dbg!(script_hash_hex);
    let configurables = claimable_predicate::ClaimableConfigurables::default()
        .with_MAKE_CLAIM_SCRIPT_HASH(get_script_bytecode_hash(&harness.script_instance).await)?
        .with_OWNER(owner_address)?;

    // PREDICATE
    let predicate_binary_path = "./out/debug/claimable.bin";
    let predicate: Predicate = Predicate::load_from(predicate_binary_path)?
        .with_provider(harness.provider.clone())
        .with_configurables(configurables);

    // FUND PREDICATE
    let claimable_amount = 10000;
    let wallet_0_amount = harness
        .provider
        .get_asset_balance(owner_wallet.address(), harness.asset_id)
        .await?;

    owner_wallet
        .transfer(
            predicate.address(),
            claimable_amount,
            harness.asset_id,
            TxPolicies::default(),
        )
        .await?;

    let mut accumulated_fee = get_last_tx_fee(&client).await;

    // BUILD TRANSACTION
    let input_coin = predicate
        .get_asset_inputs_for_amount(harness.asset_id, 1, None)
        .await?;

    //let output_coins =
    //    predicate.get_asset_outputs_for_amount(owner_wallet.address(), harness.asset_id, 0);

    //let tb = harness
    //    .script_instance
    //    .main(recipient_address, 30_000, 5, harness.asset_id.into())
    //    .with_inputs(input_coin)
    //    .with_contracts(&[&harness.contract_instance])
    //    .transaction_builder()
    //    .await
    //    .unwrap();

    //let script = tb.script;

    //let tx = harness
    //    .script_instance
    //    .main(recipient_address, 30_000, 5, harness.asset_id.into())
    //    .with_inputs(input_coin)
    //    .with_contracts(&[&harness.contract_instance])
    //    .build_tx()
    //    .await
    //    .unwrap();

    //use sha2::Digest;
    //use sha2::Sha256;

    //let mut hasher = Sha256::new();
    //hasher.update(tx.script());
    //let result = hasher.finalize();
    //let bits = Bits256(result.try_into().unwrap());
    //let h = hex::encode(bits.0);
    //println!("Bits: {h}");

    // -------------------------
    let claim_id = harness
        .script_instance
        .main(recipient_address, 30_000, 90, harness.asset_id.into())
        .with_inputs(input_coin)
        .with_contracts(&[&harness.contract_instance])
        .call()
        .await
        .unwrap()
        .value;

    accumulated_fee += get_last_tx_fee(&client).await;

    assert_eq!(claim_id, 0);

    let claims = harness
        .contract_instance
        .methods()
        .get_claims(recipient_address)
        .call()
        .await
        .unwrap()
        .value;

    assert_eq!(claims.len(), 1);
    assert_eq!(claims.get(0).unwrap().owner, owner_address);
    assert_eq!(claims.get(0).unwrap().recipient, recipient_address);
    assert_eq!(claims.get(0).unwrap().amount, 90);
    //-------------------------------------

    //ScriptTransactionBuilder::prepare_transfer(input_coin, output_coins, TxPolicies::default());

    //let call_parameters = CallParameters::default().with_amount(100);

    //let mut tb = harness
    //    .contract_instance
    //    .clone()
    //    .with_account(owner_wallet.clone())
    //    .methods()
    //    .initiate_claim(owner_wallet.address(), recipient_wallet.address())
    //    //.call_params(call_parameters)
    //    //.unwrap()
    //    .transaction_builder()
    //    .await
    //    .unwrap();

    //println!("TB: {tb:?}");
    //println!("TB inputs: {:?}", tb.inputs.len());
    //println!("TB outputs: {:?}", tb.outputs.len());
    //println!("Script: {:?}", tb.script_data);
    //println!("Script hash: {:?}", tb.script);

    //owner_wallet.adjust_for_fee(&mut tb, 0).await.unwrap();

    //println!("TB: {tb:?}");
    //println!("TB inputs: {:?}", tb.inputs.len());
    //println!("TB outputs: {:?}", tb.outputs.len());
    //println!("Script: {:?}", tb.script_data);
    //panic!("NooO");

    //let mut tb: ScriptTransactionBuilder = {
    //    let input_coin = predicate
    //        .get_asset_inputs_for_amount(harness.asset_id, 1, None)
    //        .await?;
    //    let output_coin = predicate.get_asset_outputs_for_amount(
    //        owner_wallet.address(),
    //        harness.asset_id,
    //        claimable_amount - 1,
    //    ); // minus 1 for gas

    //    ScriptTransactionBuilder::prepare_transfer(input_coin, output_coin, TxPolicies::default())
    //};

    Ok(())
}
