use leptos::prelude::*;

#[component]
pub fn make_claim() -> impl IntoView {
    let claim_funds_action = ServerAction::<ClaimFunds>::new();

    view! {
        <article>
            <header>
                <h3>"Claim funds"</h3>
            </header>
            <ActionForm action=claim_funds_action>
                <label>"owner" <input type="text" name="owner" /></label>
                <label>"asset" <input type="text" name="asset_id" /></label>
                <label>"amount" <input type="number" name="amount" /></label>
                <input type="submit" value="Claim funds" />
            </ActionForm>
        </article>
    }
}

#[server]
async fn claim_funds(owner: String, asset_id: String, amount: u64) -> Result<(), ServerFnError> {
    use crate::model;
    use crate::server::KpopServer;

    let kp: KpopServer = use_context().expect("should be able to get shared Kpop instance");

    let asset_id = if asset_id.trim().is_empty() {
        None
    } else {
        Some(asset_id.trim().to_string())
    };

    let (tx, rx) = tokio::sync::oneshot::channel();

    let owner_clone = owner.clone();
    let asset_id_clone = asset_id.clone();
    let kp_clone = kp.clone();
    leptos::task::spawn_local(async move {
        let res = kp_clone.claim(&owner_clone, asset_id_clone, amount).await;
        tx.send(res).expect("should be able to send result");
    });

    let claim_id = rx.await.expect("should be able to receive result");

    let recipient = kp.wallet_address().await;

    let block_height = kp.block_height().await;

    let asset_id = asset_id.unwrap_or(kp.base_asset_id());

    let claim = model::Claim {
        claim_id,
        owner,
        recipient,
        asset_id,
        amount,
        block_height,
    };

    kp.insert_active_claim(claim);

    Ok(())
}
