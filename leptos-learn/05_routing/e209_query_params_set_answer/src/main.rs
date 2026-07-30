// ============================================================
// Exercise 209 - Answer: Setting Query Parameters
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
fn SearchForm() -> impl IntoView {
    let navigate = use_navigate();
    let (input, set_input) = signal(String::new());

    view! {
        <nav>
            <input
                type="text"
                placeholder="输入搜索关键词"
                on:input=move |ev| {
                    set_input(event_target_value(&ev));
                }
                prop:value=input
            />
            <button on:click=move |_| {
                let q = input.get();
                if !q.is_empty() {
                    navigate(&format!("/search?q={}", q), Default::default());
                }
            }>"搜索"</button>
            " "
            <button on:click=move |_| navigate("/search?q=leptos", Default::default())>
                "搜索 Leptos"
            </button>
        </nav>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <Router>
            <SearchForm/>
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
