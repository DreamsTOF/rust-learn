// ============================================================
// Exercise e321 — preload_data — Answer
//
// Core: preload_data pattern, Resource + Suspense for SSR
// ============================================================

use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    // Server-side preload: Resource fetches data during SSR rendering.
    // The source (|| ()) runs once; the fetcher runs on the server.
    let data = Resource::new(
        || (),                                      // source signal (no reactive deps here)
        |_| async { "Hello from preloaded data!" }, // fetcher — runs during SSR
    );

    view! {
        <div>
            <h2>"Preload Data Pattern"</h2>
            <Suspense fallback=|| view! { <p>"Loading..."</p> }>
                <p>{data.map(|d| d.map(|v| v.to_string()))}</p>
            </Suspense>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
