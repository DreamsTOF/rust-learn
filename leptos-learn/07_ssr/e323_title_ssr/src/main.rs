use leptos::prelude::*;

// TODO: Use <Title/> to set dynamic page title per route/component
//
// Core: <Title text="..."/> component, dynamic title in SSR
//
// Hints:
//   1. Define #[component] fn Exercise() -> impl IntoView
//   2. Use <Title text="..."/> from leptos::meta to set document <title>
//   3. Try making the title dynamic with a signal
//   4. Import Title from leptos::meta

#[component]
fn Exercise() -> impl IntoView {
    // TODO: create a signal for dynamic title text
    // let (title, set_title) = signal("Leptos SSR - Dynamic Title");

    view! {
        <div>
            <h2>"Dynamic Title (SSR)"</h2>
            // TODO: Add <Title text=.../> here — use signal for dynamic updates
            // TODO: Add an <input> to change the title on user input
            <p>"练习 323 (title_ssr)"</p>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
