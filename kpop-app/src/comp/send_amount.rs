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
                <input placeholder="recipient" type="text" name="recipient" />
                <input placeholder="asset" type="text" name="asset_id" />
                <input placeholder="amount" type="number" name="amount" />
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
