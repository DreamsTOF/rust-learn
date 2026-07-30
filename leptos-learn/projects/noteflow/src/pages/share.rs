use crate::components::editor::render_markdown;
use crate::hooks::use_doc_tree::find_doc_by_id;
use crate::state::AppState;
use leptos::prelude::*;

#[component]
pub fn SharePage() -> impl IntoView {
    let state = use_context::<AppState>().expect("AppState not provided");
    let params = leptos_router::hooks::use_params_map();
    let doc_id = move || params.get().get("id").map(|s| s.clone()).unwrap_or_default();

    let doc = move || {
        let docs = state.docs.get();
        find_doc_by_id(&docs, &doc_id())
    };

    let html_content = move || {
        doc().map(|d| render_markdown(&d.content)).unwrap_or_default()
    };

    view! {
        <div class="share-page">
            {move || {
                match doc() {
                    Some(d) => {
                        view! {
                            <div class="share-container">
                                <header class="share-header">
                                    <h1>{d.title.clone()}</h1>
                                    <div class="share-meta">
                                        <span>"共享文档"</span>
                                        <span>
                                            {chrono::DateTime::from_timestamp(d.updated_at, 0)
                                                .map(|dt| dt.format("%Y-%m-%d %H:%M").to_string())
                                                .unwrap_or_default()}
                                        </span>
                                    </div>
                                </header>
                                <article class="share-content" inner_html=html_content></article>
                            </div>
                        }
                    }.into_any(),
                    None => {
                        view! {
                            <div class="share-not-found">
                                <h1>"文档未找到"</h1>
                                <p>"该分享链接无效或文档已被删除。"</p>
                            </div>
                        }.into_any()
                    }
                }
            }}
        </div>
    }
}
