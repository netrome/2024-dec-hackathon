use leptos::prelude::*;

#[component]
pub fn send_form() -> impl IntoView {
    let send_amount_action = ServerAction::<SendTo>::new();
    view! {
        <article>
            <header>
                <h3>"Send to"</h3>
            </header>
            <ActionForm action=send_amount_action>
                <label>"recipient" <input type="text" name="recipient" /></label>
                <label>"asset" <input type="text" name="asset_id" /></label>
                <label>"amount" <input type="number" name="amount" /></label>
                <input type="submit" value="Send funds" />
            </ActionForm>
        </article>
    }
}

#[server]
async fn send_to(recipient: String, asset_id: String, amount: u64) -> Result<(), ServerFnError> {
    use crate::server::KpopServer;

    let kp: KpopServer = use_context().expect("should be able to get shared Kpop instance");

    let asset_id = if asset_id.trim().is_empty() {
        None
    } else {
        Some(asset_id.trim().to_string())
    };

    kp.send_to(&recipient, asset_id, amount).await;

    Ok(())
}
