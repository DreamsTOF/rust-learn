// ============================================================
// Exercise 236 - Answer
// ============================================================

use leptos::prelude::*;
use leptos_meta::Html;

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <Html {..} lang="en" class="my-app" />
        <div>
            <p>"HTML <html> attributes set via leptos_meta::Html"</p>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
