use crate::model;
use leptos::prelude::*;

#[component]
pub fn wallet_info() -> impl IntoView {
    let info = Resource::new(|| (), |_| get_wallet_info());

    view! {
        <Suspense fallback = || view!{<p>"Loading..."</p>}>
        <ErrorBoundary
            fallback = |_errors| {
                view! {<p>"Uh oh - we got an error"</p>}
            }
        >
            <p>"yo"</p>
            {
                move || info.get().map(|res| res.map(|kpop_info| view!{<KpopInfo kpop_info/>}))
            }
        </ErrorBoundary>
        </Suspense>
    }
}

#[component]
fn kpop_info(kpop_info: model::KpopInfo) -> impl IntoView {
    view! {
        <article>
            <header>
                <p>"Kpop info goes here..."</p>
                <p>{kpop_info.base_address}</p>
                <p>{kpop_info.claimable_address}</p>
                <p>{kpop_info.provider_url}</p>
                <p>{kpop_info.contract_id}</p>
            </header>
        </article>
    }
}

#[server]
async fn get_wallet_info() -> Result<model::KpopInfo, ServerFnError> {
    use crate::shared::SharedKpop;
    let kp: SharedKpop = use_context().expect("should be able to get shared Kpop instance");
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
