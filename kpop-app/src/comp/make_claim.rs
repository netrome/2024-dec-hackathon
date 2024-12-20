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
    use crate::shared::SharedKpop;

    let kp: SharedKpop = use_context().expect("should be able to get shared Kpop instance");

    let asset_id = if asset_id.trim().is_empty() {
        None
    } else {
        Some(asset_id.trim().to_string())
    };

    kp.claim(&owner, asset_id, amount).await;

    Ok(())
}
