use std::collections::HashMap;

use leptos::prelude::*;

#[component]
pub fn balance_table(balances: HashMap<String, u64>) -> impl IntoView {
    let s = format!("{balances:?}");

    view! {
        <p>{s}</p>
    }
}
