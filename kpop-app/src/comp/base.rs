use leptos::prelude::*;

#[component]
pub fn base(children: ChildrenFn) -> impl IntoView {
    view! { <main class="container">{children()}</main> }
}
