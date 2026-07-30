use crate::state::AppState;
use leptos::prelude::*;
use wasm_bindgen::prelude::*;

#[component]
pub fn SearchPanel() -> impl IntoView {
    let state = use_context::<AppState>().expect("AppState not provided");
    let query = RwSignal::new(String::new());
    let is_open = RwSignal::new(false);
    let results = RwSignal::new(Vec::<(String, String, bool)>::new());

    let do_search = move |q: &str| {
        if q.is_empty() {
            results.set(Vec::new());
            return;
        }
        let q_lower = q.to_lowercase();
        let docs = state.docs.get();
        let mut found = Vec::new();
        for doc in docs.iter() {
            if doc.title.to_lowercase().contains(&q_lower) {
                found.push((doc.id.clone(), doc.title.clone(), true));
            } else if doc.content.to_lowercase().contains(&q_lower) {
                found.push((doc.id.clone(), doc.title.clone(), false));
            }
        }
        // TODO: 练习 - 搜索结果按匹配度排序，标题匹配优先于内容匹配
        // 提示: 使用 sort_by 按元组第三个元素 (bool) 降序排列，并限制最多 20 条结果
        found.sort_by(|a, b| b.2.cmp(&a.2));
        found.truncate(20);
        results.set(found);
    };

    let on_input = move |ev: leptos::ev::Event| {
        let val = event_target_value(&ev);
        query.set(val.clone());
        do_search(&val);
    };

    let handle_blur = move || {
        let is_open_clone = is_open;
        let cb = Closure::wrap(Box::new(move || {
            is_open_clone.set(false);
        }) as Box<dyn FnMut()>);
        if let Some(window) = web_sys::window() {
            let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                cb.as_ref().unchecked_ref(),
                200,
            );
        }
        cb.forget();
    };

    view! {
        <div class="search-panel">
            <div class="search-input-wrapper">
                <input
                    type="text"
                    class="search-input"
                    placeholder="搜索文档... (Ctrl+K)"
                    prop:value=query
                    on:input=on_input
                    on:focus=move |_| is_open.set(true)
                    on:blur=move |_| handle_blur()
                />
            </div>
            {move || {
                let open = is_open.get();
                let results_clone = results.get();
                if open && !results_clone.is_empty() {
                    let state = state.clone();
                    view! {
                        <div class="search-results">
                            <For
                                each=move || results.get()
                                key=|(id, _, _)| id.clone()
                                children=move |(id, title, is_match)| {
                                    let state = state.clone();
                                    view! {
                                        <div
                                            class="search-result-item"
                                            on:mousedown=move |_| {
                                                state.selected_doc_id.set(Some(id.clone()));
                                                state.active_tab_id.set(Some(id.clone()));
                                            }
                                        >
                                            <span class="result-title">{title.clone()}</span>
                                            <span class="result-type">
                                                {if is_match { "标题匹配" } else { "内容匹配" }}
                                            </span>
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
