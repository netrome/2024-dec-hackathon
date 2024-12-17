use fuels::{
    accounts::{predicate::Predicate, wallet::WalletUnlocked, Account},
    client::{FuelClient, PaginationRequest},
    crypto::SecretKey,
    prelude::*,
    types::transaction_builders::{BuildableTransaction, ScriptTransactionBuilder},
};

use fuel_core_client::client::types::TransactionStatus;

abigen!(Predicate(
    name = "Claimable",
    abi = "out/debug/claimable-abi.json"
));

mod claims_contract {
    use fuels::prelude::*;

    abigen!(Contract(
        name = "ClaimsContract",
        abi = "../claims-contract/out/debug/claims-contract-abi.json"
    ));
}

async fn setup_wallets_and_network() -> (Vec<WalletUnlocked>, Provider, AssetId) {
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

    return (vec![wallet_0, wallet_1, wallet_2], provider, asset_id);
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
    let (wallets, provider, asset_id) = setup_wallets_and_network().await;
    let client = FuelClient::new(provider.url()).unwrap();

    // CONFIGURABLES
    let owner_wallet = wallets.get(0).expect("no owner wallet");
    let owner_address: Address = wallets[0].address().into();

    let configurables = ClaimableConfigurables::default()
        .with_CLAIMS_CONTRACT_ADDRESS(Address::zeroed())?
        .with_OWNER(owner_address)?;

    // PREDICATE
    let predicate_binary_path = "./out/debug/claimable.bin";
    let predicate: Predicate = Predicate::load_from(predicate_binary_path)?
        .with_provider(provider.clone())
        .with_configurables(configurables);

    // FUND PREDICATE
    let claimable_amount = 100;
    let wallet_0_amount = provider
        .get_asset_balance(wallets[0].address(), asset_id)
        .await?;

    owner_wallet
        .transfer(
            predicate.address(),
            claimable_amount,
            asset_id,
            TxPolicies::default(),
        )
        .await?;
    let mut accumulated_fee = get_last_tx_fee(&client).await;

    // BUILD TRANSACTION
    let mut tb: ScriptTransactionBuilder = {
        let input_coin = predicate
            .get_asset_inputs_for_amount(asset_id, 1, None)
            .await?;
        let output_coin = predicate.get_asset_outputs_for_amount(
            wallets[0].address().into(),
            asset_id,
            claimable_amount - 1,
        ); // minus 1 for gas

        ScriptTransactionBuilder::prepare_transfer(input_coin, output_coin, TxPolicies::default())
    };

    // SIGN TRANSACTION
    tb.add_signer(owner_wallet.clone())?;

    assert_eq!(
        provider
            .get_asset_balance(predicate.address(), asset_id)
            .await?,
        claimable_amount
    );
    assert_eq!(
        provider
            .get_asset_balance(owner_wallet.address(), asset_id)
            .await?,
        wallet_0_amount - claimable_amount - accumulated_fee
    );

    // SPEND PREDICATE
    let tx: ScriptTransaction = tb.build(provider.clone()).await?;
    provider.send_transaction_and_await_commit(tx).await?;
    accumulated_fee += get_last_tx_fee(&client).await;

    assert_eq!(
        provider
            .get_asset_balance(predicate.address(), asset_id)
            .await?,
        0
    );
    assert_eq!(
        provider
            .get_asset_balance(owner_wallet.address(), asset_id)
            .await?,
        wallet_0_amount - accumulated_fee
    );

    Ok(())
}

#[tokio::test]
async fn recepient_can_initiate_a_claim_from_a_claimable_predicate() -> Result<()> {
    todo!();
}
