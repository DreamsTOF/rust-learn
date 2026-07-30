use crate::hooks::use_toc::extract_toc;
use crate::state::AppState;
use leptos::prelude::*;

#[component]
pub fn TocPanel() -> impl IntoView {
    let state = use_context::<AppState>().expect("AppState not provided");
    let is_collapsed = RwSignal::new(false);

    let toc_items = move || {
        let docs = state.docs.get();
        let active_id = state.active_tab_id.get();
        if let Some(id) = active_id {
            if let Some(doc) = docs.iter().find(|d| d.id == id) {
                return extract_toc(&doc.content);
            }
        }
        Vec::new()
    };

    view! {
        <div class="toc-panel">
            <div class="toc-header" on:click=move |_| is_collapsed.update(|v| *v = !*v)>
                <h4>"目录"</h4>
                <span>{move || if is_collapsed.get() { "▶" } else { "▼" }}</span>
            </div>
            {move || {
                if !is_collapsed.get() {
                    view! {
                        <div class="toc-list">
                            <For
                                each=move || toc_items()
                                key=|item| item.anchor.clone()
                                children=move |item| {
                                    let padding = format!("padding-left: {}px", (item.level - 1) * 16);
                                    view! {
                                        <div
                                            class="toc-item"
                                            style=padding
                                            on:click=move |_| {
                                                // Scroll to heading
                                                if let Some(window) = web_sys::window() {
                                                    if let Some(doc_el) = window.document() {
                                                        if let Some(el) = doc_el.get_element_by_id(&item.anchor) {
                                                            let _ = el.scroll_into_view();
                                                        }
                                                    }
                                                }
                                            }
                                        >
                                            {item.text.clone()}
                                        </div>
                                    }
                                }
                            />
                        </div>
                    }.into_any()
                } else {
                    view! { <div></div> }.into_any()
                }
            }}
        </div>
    }
}
