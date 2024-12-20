use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

use crate::comp::claims::Claims;
use crate::comp::fund::FundForm;
use crate::comp::make_claim::MakeClaim;
use crate::comp::predicate_balances::PredicateBalances;
use crate::comp::send_amount::SendForm;
use crate::comp::wallet_balances::WalletBalances;
use crate::comp::wallet_info::WalletInfo;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en" >
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <link rel="stylesheet" href="/static/pico.fuchsia.min.css" />
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/kpop-app.css"/>

        // sets the document title
        <Title text="Kpop wallet"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    view! {
        <article>
            <h1>"KPOP WALLET"</h1>
            <p><i>"- Never lose your funds."</i></p>
        </article>
        <div class="grid">
            <div>
                <WalletInfo />
            </div>
            <div>
                <WalletBalances />
                <PredicateBalances />
            </div>
        </div>
        <div class="grid">
            <div>
                <FundForm />
            </div>
            <div>
                <SendForm />
            </div>
            <div>
                <MakeClaim />
            </div>
        </div>
        <Claims />
    }
}
