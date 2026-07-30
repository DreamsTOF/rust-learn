// ============================================================
// Exercise 247 - Answer: Route Tab Persistence
// ============================================================

use std::collections::HashMap;
use leptos::prelude::*;
use leptos::html::Div;
use leptos_router::components::*;
use leptos_router::hooks::*;
use leptos_router::NavigateOptions;
use leptos_router::path;

const TABS: &[(&str, &str)] = &[
    ("tab1", "Tab 1"),
    ("tab2", "Tab 2"),
    ("tab3", "Tab 3"),
];

/// Generate a long list for scroll testing.
fn long_list(prefix: &str) -> Vec<String> {
    (1..=50).map(|i| format!("{} — item #{}", prefix, i)).collect()
}

#[component]
fn TabPersistence() -> impl IntoView {
    let query = use_query_map();
    let navigate = use_navigate();

    // Currently active tab id, defaults to "tab1".
    let active = move || {
        query
            .read()
            .get("tab")
            .unwrap_or_default()
    };

    // One NodeRef per tab for scroll position control.
    let tab_refs = RwSignal::new(HashMap::<String, NodeRef<Div>>::new());
    {
        let mut refs = tab_refs.write();
        for (id, _) in TABS {
            refs.insert(id.to_string(), NodeRef::<Div>::new());
        }
    }

    // Persisted scroll positions: { tab_id → scrollTop }.
    let saved_scrolls = RwSignal::new(HashMap::<String, i32>::new());

    // Restore scroll position when active tab changes.
    Effect::new(move || {
        let current = active();
        request_animation_frame(move || {
            if let Some(node_ref) = tab_refs.read().get(&current) {
                if let Some(el) = node_ref.get() {
                    let saved = saved_scrolls.read().get(&current).copied().unwrap_or(0);
                    el.set_scroll_top(saved);
                }
            }
        });
    });

    let save_current_scroll = move || {
        let current = active();
        if let Some(node_ref) = tab_refs.read().get(&current) {
            if let Some(el) = node_ref.get() {
                saved_scrolls.update(|m| {
                    m.insert(current, el.scroll_top());
                });
            }
        }
    };

    view! {
        <div style="font-family: system-ui, sans-serif; max-width: 700px; margin: 0 auto; padding: 1rem;">
            <h2>"Tab Persistence"</h2>

            <div style="display: flex; gap: 0;">
                {TABS
                    .iter()
                    .map(|(id, label)| {
                        let id_str = *id;
                        let is_active = move || active() == id_str;
                        let nav = navigate.clone();
                        view! {
                            <button
                                class="tab-btn"
                                class:active=is_active
                                on:click=move |_| {
                                    save_current_scroll();
                                    nav(&format!("/?tab={}", id_str), NavigateOptions::default());
                                }
                            >
                                {*label}
                            </button>
                        }
                    })
                    .collect_view()}
            </div>

            {TABS
                .iter()
                .map(|(id, label)| {
                    let id_str = *id;
                    let items = long_list(label);
                    let node_ref = tab_refs.read().get(id_str).unwrap().clone();
                    let is_visible = move || active() == id_str;

                    view! {
                        <div
                            class="tab-content"
                            style:display=move || {
                                if is_visible() { "block" } else { "none" }
                            }
                            node_ref=node_ref
                        >
                            <h3>{*label}</h3>
                            {items
                                .into_iter()
                                .map(|item| {
                                    view! { <div class="item">{item}</div> }
                                })
                                .collect_view()}
                        </div>
                    }
                })
                .collect_view()}
        </div>
    }
}

#[component]
fn App() -> impl IntoView {
    view! {
        <Router>
            <Routes fallback=|| "Not found.">
                <Route path=path!("") view=TabPersistence />
            </Routes>
        </Router>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}
