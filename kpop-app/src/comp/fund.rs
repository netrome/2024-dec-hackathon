use leptos::prelude::*;

#[component]
pub fn fund_form() -> impl IntoView {
    let fund_predicate_action = ServerAction::<FundPredicate>::new();
    view! {
        <article>
            <header>
                <h3>"Fund predicate"</h3>
            </header>
            <ActionForm action=fund_predicate_action>
                <label>"asset" <input type="text" name="asset_id" /></label>
                <label>"amount" <input type="number" name="amount" /></label>
                <input type="submit" value="Fund predicate" />
            </ActionForm>
        </article>
    }
}

#[server]
async fn fund_predicate(asset_id: String, amount: u64) -> Result<(), ServerFnError> {
    use crate::shared::SharedKpop;

    let kp: SharedKpop = use_context().expect("should be able to get shared Kpop instance");

    let asset_id = if asset_id.trim().is_empty() {
        None
    } else {
        Some(asset_id.trim().to_string())
    };

    kp.fund_predicate(asset_id, amount).await;

    Ok(())
}
