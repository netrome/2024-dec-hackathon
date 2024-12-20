use crate::model;
use leptos::prelude::*;

#[component]
pub fn wallet_info() -> impl IntoView {
    let info = Resource::new(|| (), |_| get_wallet_info());

    view! {
        <Suspense fallback=|| view! { <p>"Loading..."</p> }>
            <ErrorBoundary fallback=|_errors| {
                view! { <p>"Uh oh - we got an error"</p> }
            }>
                {move || {
                    info.get().map(|res| res.map(|kpop_info| view! { <KpopInfo kpop_info /> }))
                }}
            </ErrorBoundary>
        </Suspense>
    }
}

#[component]
fn kpop_info(kpop_info: model::KpopInfo) -> impl IntoView {
    view! {
        <article>
            <header>
                <h3>"My info"</h3>
            </header>
            <ul>
                <li>
                    <b class="float-left">"Base address: "</b>
                    <br />
                    <span class="float-right">{kpop_info.base_address}</span>
                </li>
                <li>
                    <b class="float-left">"Claimable address "</b>
                    <br />
                    <span class="float-right">{kpop_info.claimable_address}</span>
                </li>
                <li>
                    <b class="float-left">"Provider url: "</b>
                    <br />
                    <span class="float-right">{kpop_info.provider_url}</span>
                </li>
                <li>
                    <b class="float-left">"Claims contract id: "</b>
                    <br />
                    <span class="float-right">{kpop_info.contract_id}</span>
                </li>
            </ul>
        </article>
    }
}

#[server]
async fn get_wallet_info() -> Result<model::KpopInfo, ServerFnError> {
    use crate::server::KpopServer;
    let kp: KpopServer = use_context().expect("should be able to get shared Kpop instance");

    let base_address = kp.wallet_address().await;
    let claimable_address = kp.predicate_address().await;
    let contract_id = kp.contract_id();
    let provider_url = kp.provider_url();

    let kpop_info = model::KpopInfo {
        base_address,
        claimable_address,
        contract_id,
        provider_url,
    };

    Ok(kpop_info)
}
