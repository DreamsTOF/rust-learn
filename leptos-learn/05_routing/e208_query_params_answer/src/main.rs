// ============================================================
// Exercise 208 - Answer: Reading Query Parameters with use_query_map()
// ============================================================

use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::hooks::*;
use leptos_router::path;

#[component]
fn SearchPage() -> impl IntoView {
    let query = use_query_map();
    let search_term = move || {
        query()
            .get("q")
            .cloned()
            .unwrap_or_default()
    };

    view! {
        <h2>"搜索页面"</h2>
        <p>"当前搜索词: " {search_term}</p>
    }
}

#[component]
fn SearchButtons() -> impl IntoView {
    let navigate = use_navigate();
    let navigate_rust = navigate.clone();

    view! {
        <nav>
            <button on:click=move |_| navigate("/search?q=leptos", Default::default())>
                "搜索 Leptos"
            </button>
            " "
            <button on:click=move |_| navigate_rust("/search?q=rust", Default::default())>
                "搜索 Rust"
            </button>
        </nav>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <Router>
            <SearchButtons/>
            <main>
                <Routes fallback=|| "页面未找到">
                    <Route path=path!("/search") view=SearchPage/>
                </Routes>
            </main>
        </Router>
    }
}

fn main() {
    mount_to_body(Exercise);
}
