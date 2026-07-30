// ============================================================
// Exercise 239 - Answer
// ============================================================

use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::hooks::*;
use leptos_router::path;

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <Router>
            <nav>
                <A href="/">"Home"</A>
                <A href="/data/42">"Data 42"</A>
                <A href="/data/99">"Data 99"</A>
            </nav>
            <main>
                <Routes fallback=|| "Not found">
                    <Route path=path!("/") view=|| view! { <p>"Home"</p> }/>
                    <Route path=path!("/data/:id") view=DataPage/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn DataPage() -> impl IntoView {
    let params = use_params_map();
    let id = move || params().get("id").unwrap_or_default();

    view! {
        <Await
            future=async move {
                format!("Loaded data record for ID: {}", id())
            }
            let:data
        >
            <p>{data.clone()}</p>
        </Await>
    }
}

fn main() {
    mount_to_body(Exercise);
}
