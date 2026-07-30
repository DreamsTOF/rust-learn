// ============================================================
// Exercise 248 - Answer: Route Search Sync
// ============================================================

use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::hooks::*;
use leptos_router::NavigateOptions;
use leptos_router::path;

/// Simple URL encoding for query parameter values.
fn urlencoding(s: &str) -> String {
    let mut result = String::new();
    for c in s.chars() {
        match c {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_' | '.' | '~' => result.push(c),
            ' ' => result.push('+'),
            _ => {
                let mut buf = [0u8; 4];
                let encoded_str = c.encode_utf8(&mut buf);
                for byte in encoded_str.as_bytes() {
                    result.push_str(&format!("%{:02X}", byte));
                }
            }
        }
    }
    result
}

#[component]
fn SearchPage() -> impl IntoView {
    let query = use_query_map();
    let navigate = use_navigate();

    // Read initial search term from URL.
    let initial = query.read().get("q").unwrap_or_default();
    let (search_text, set_search_text) = signal(initial);

    // Direction ①: URL → input — sync URL query to input when it changes externally.
    Effect::new(move || {
        let url_value = query.read().get("q").unwrap_or_default();
        let current = search_text.get();
        if url_value != current {
            set_search_text.set(url_value);
        }
    });

    // Direction ②: input → URL — update URL query on user input.
    let on_input = move |ev| {
        let new_val = event_target_value(&ev);
        set_search_text.set(new_val.clone());

        let encoded = urlencoding(&new_val);
        navigate(&format!("/?q={}", encoded), NavigateOptions::default());
    };

    // Simulated search results.
    let results = move || {
        let q = search_text.get();
        if q.trim().is_empty() {
            vec!["Start typing to search...".to_string()]
        } else {
            (1..=5)
                .map(|i| format!("Result {} for \"{}\"", i, q))
                .collect()
        }
    };

    view! {
        <div style="font-family: system-ui, sans-serif; max-width: 600px; margin: 0 auto; padding: 1rem;">
            <h2>"Search Sync"</h2>

            <div style="margin-bottom: 1rem;">
                <input
                    type="search"
                    placeholder="Type to search..."
                    prop:value=move || search_text.get()
                    on:input=on_input
                    style="width: 100%; padding: 0.75rem; font-size: 1rem; border: 2px solid #ddd; border-radius: 6px; box-sizing: border-box;"
                />
            </div>

            <div>
                <p style="color: #666; font-size: 0.9rem;">
                    <em>
                        "URL: " <code>{move || format!("/?q={}", search_text.get())}</code>
                    </em>
                </p>
                <ul>
                    {move || {
                        results()
                            .into_iter()
                            .map(|r| view! { <li style="padding: 0.25rem 0;">{r}</li> })
                            .collect_view()
                    }}
                </ul>
            </div>

            <hr />
            <h3>"Try it:"</h3>
            <ol>
                <li>"Type in the search box — URL updates."</li>
                <li>"Use browser back/forward — input updates."</li>
            </ol>
        </div>
    }
}

#[component]
fn App() -> impl IntoView {
    view! {
        <Router>
            <Routes fallback=|| "Not found.">
                <Route path=path!("") view=SearchPage />
            </Routes>
        </Router>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}
