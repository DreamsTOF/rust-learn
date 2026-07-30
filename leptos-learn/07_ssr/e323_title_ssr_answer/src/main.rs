// ============================================================
// Exercise e323 — title_ssr — Answer
//
// Core: <Title/> component, dynamic title per route
// ============================================================

use leptos::prelude::*;
use leptos::meta::*;

#[component]
fn Exercise() -> impl IntoView {
    let (title, set_title) = signal("Leptos SSR — Dynamic Title".to_string());

    view! {
        <div>
            <h2>"Dynamic Title (SSR)"</h2>

            // Sets <title> in document <head> — reactive to signal changes
            <Title text=title />

            <input
                prop:value=title
                on:input=move |ev| set_title.set(event_target_value(&ev))
            />
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
