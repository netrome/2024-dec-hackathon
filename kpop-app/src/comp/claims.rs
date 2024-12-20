use leptos::{either::Either, prelude::*};

use crate::model;

#[component]
pub fn claims() -> impl IntoView {
    let claims_on_user = Resource::new(|| (), |_| get_claims());
    let user_claims = Resource::new(|| (), |_| get_user_claims());

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
                        claims_on_user
                            .get()
                            .map(|res| {
                                res.map(|claims| view! { <ClaimsTable claims is_user=false /> })
                            })
                    }}
                </article>
                <article>
                    <header>
                        <h3>"My claims"</h3>
                    </header>
                    {move || {
                        user_claims
                            .get()
                            .map(|res| {
                                res.map(|claims| view! { <ClaimsTable claims is_user=true /> })
                            })
                    }}
                </article>
            </ErrorBoundary>
        </Suspense>
    }
}

#[component]
pub fn claims_table(claims: Vec<model::Claim>, is_user: bool) -> impl IntoView {
    let (active_idx, set_active_idx) = signal(usize::MAX);

    let claim_rows = move || -> Vec<_> {
        claims
            .clone()
            .into_iter()
            .enumerate()
            .map(|(idx, claim)| {
                let is_active = active_idx.get() == idx;
                if is_user {
                    Either::Left(view! { <InboundClaimRow claim idx is_active set_active_idx /> })
                } else {
                    Either::Right(view! { <OutboundClaimRow claim idx is_active set_active_idx /> })
                }
            })
            .collect()
    };

    view! {
        <div class="overflow-auto">
            <table>
                <thead>
                    <tr>
                        <th></th>
                        <th>"ID"</th>
                        <th>"Block Height"</th>
                        <th>"Asset ID"</th>
                        <th>"Amount"</th>
                        <th>"Recipient"</th>
                        <th>"Owner"</th>
                    </tr>
                </thead>
                <tbody>{claim_rows}</tbody>
            </table>
        </div>
    }
}

#[component]
pub fn outbound_claim_row(
    claim: model::Claim,
    idx: usize,
    is_active: bool,
    set_active_idx: WriteSignal<usize>,
) -> impl IntoView {
    let toggle_active = move |_| {
        if is_active {
            set_active_idx.set(usize::MAX);
        } else {
            set_active_idx.set(idx);
        }
    };

    let disprove_action = ServerAction::<DisproveClaim>::new();

    let disprove_button = if is_active {
        Some(view! {
            <button on:click=move |_| {
                disprove_action.dispatch(claim.claim_id.into());
            }>"disprove"</button>
        })
    } else {
        None
    };

    view! {
        <tr on:click=toggle_active>
            <td>{disprove_button}</td>
            <td>{claim.claim_id}</td>
            <td>{claim.block_height}</td>
            <td>{claim.asset_id}</td>
            <td>{claim.amount}</td>
            <td>{claim.recipient}</td>
            <td>{claim.owner}</td>
        </tr>
    }
}

#[component]
pub fn inbound_claim_row(
    claim: model::Claim,
    idx: usize,
    is_active: bool,
    set_active_idx: WriteSignal<usize>,
) -> impl IntoView {
    let toggle_active = move |_| {
        if is_active {
            set_active_idx.set(usize::MAX);
        } else {
            set_active_idx.set(idx);
        }
    };

    let fulfill_action = ServerAction::<FulfillClaim>::new();

    let fulfill_button = if is_active {
        Some(view! {
            <button on:click=move |_| {
                fulfill_action.dispatch(claim.claim_id.into());
            }>"fulfill"</button>
        })
    } else {
        None
    };

    view! {
        <tr on:click=toggle_active>
            <td>{fulfill_button}</td>
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
    use crate::server::KpopServer;
    let kp: KpopServer = use_context().expect("should be able to get shared Kpop instance");

    Ok(kp.get_claims().await)
}

#[server]
async fn get_user_claims() -> Result<Vec<model::Claim>, ServerFnError> {
    use crate::server::KpopServer;
    let kp: KpopServer = use_context().expect("should be able to get shared Kpop instance");

    Ok(kp.get_active_claims())
}

#[server]
async fn disprove_claim(id: u64) -> Result<(), ServerFnError> {
    use crate::server::KpopServer;
    let kp: KpopServer = use_context().expect("should be able to get shared Kpop instance");

    kp.disprove_claim(id).await;

    Ok(())
}

#[server]
async fn fulfill_claim(id: u64) -> Result<(), ServerFnError> {
    use crate::server::KpopServer;
    let kp: KpopServer = use_context().expect("should be able to get shared Kpop instance");

    kp.fulfill_claim(id).await;

    Ok(())
}
