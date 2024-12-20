use std::collections::HashMap;

use leptos::prelude::*;

#[component]
pub fn predicate_balances() -> impl IntoView {
    let info = Resource::new(|| (), |_| get_predicate_balances());
    view! {
        <Suspense fallback=|| view! { <p>"Loading..."</p> }>
            <ErrorBoundary fallback=|_errors| {
                view! { <p>"Uh oh - we got an error"</p> }
            }>
                <article>
                    <header>
                        <h3>"Claimable balance"</h3>
                    </header>
                    {move || {
                        info.get()
                            .map(|res| res.map(|balances| view! { <BalanceTable balances /> }))
                    }}
                </article>
            </ErrorBoundary>
        </Suspense>
    }
}

#[component]
pub fn balance_table(balances: HashMap<String, u64>) -> impl IntoView {
    let s = format!("{balances:?}");

    view! { <p>{s}</p> }
}

#[server]
async fn get_predicate_balances() -> Result<HashMap<String, u64>, ServerFnError> {
    use crate::server::KpopServer;
    let kp: KpopServer = use_context().expect("should be able to get shared Kpop instance");

    Ok(kp.predicate_balance().await)
}
