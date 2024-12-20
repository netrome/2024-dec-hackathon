use leptos::prelude::*;

use crate::model;

#[component]
pub fn claims() -> impl IntoView {
    let our_claims = Resource::new(|| (), |_| get_claims());

    view! {
        <Suspense fallback=|| view! { <p>"Loading..."</p> }>
            <ErrorBoundary fallback=|_errors| {
                view! { <p>"Uh oh - we got an error"</p> }
            }>
                <article>
                <header>
                    <h3>"Claims on my balance"</h3>
                </header>
                {move || {
                    our_claims.get().map(|res| res.map(|claims| view! { <ClaimsTable claims/> }))
                }}
                </article>
            </ErrorBoundary>
        </Suspense>
    }
}

#[component]
pub fn claims_table(claims: Vec<model::Claim>) -> impl IntoView {
    let claim_rows: Vec<_> = claims
        .into_iter()
        .map(|claim| view! {<ClaimRow claim />})
        .collect();

    view! {
        <table>
            <tr>
                <th>"ID"</th>
                <th>"Block Height"</th>
                <th>"Asset ID"</th>
                <th>"Amount"</th>
                <th>"Recipient"</th>
                <th>"Owner"</th>
            </tr>
            { claim_rows }
        </table>
    }
}

#[component]
pub fn claim_row(claim: model::Claim) -> impl IntoView {
    view! {
        <tr>
            <td>{claim.claim_id}</td>
            <td>{claim.block_height}</td>
            <td>{claim.asset_id}</td>
            <td>{claim.amount}</td>
            <td>{claim.recipient}</td>
            <td>{claim.owner}</td>
        </tr>
    }
}

#[server]
async fn get_claims() -> Result<Vec<model::Claim>, ServerFnError> {
    use crate::shared::SharedKpop;
    let kp: SharedKpop = use_context().expect("should be able to get shared Kpop instance");

    Ok(kp.get_claims().await)
}

#[server]
async fn disprove_claim(id: u64) -> Result<(), ServerFnError> {
    use crate::shared::SharedKpop;
    let kp: SharedKpop = use_context().expect("should be able to get shared Kpop instance");

    kp.disprove_claim(id).await;

    Ok(())
}
