use crate::components::editor::Editor;
use crate::components::search_panel::SearchPanel;
use crate::components::template_picker::TemplatePicker;
use crate::components::toc_panel::TocPanel;
use crate::components::link_graph::LinkGraph;
use crate::components::comment_panel::CommentPanel;
use crate::components::version_history::VersionHistory;
use crate::state::AppState;
use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

#[component]
pub fn EditorPage() -> impl IntoView {
    let state = use_context::<AppState>().expect("AppState not provided");
    let params = use_params_map();
    let doc_id = move || {
        params.get().get("id").map(|s| s.clone())
    };

    // TODO: 练习 - 实现文档加载逻辑
    // 提示: 检查 doc_id 是否有效，如果提供了 doc_id 但文档不存在则显示"文档未找到"
    let doc_not_found = move || {
        if let Some(id) = doc_id() {
            let docs = state.docs.get();
            !docs.iter().any(|d| d.id == id)
        } else {
            false
        }
    };

    let welcome = move || doc_id().is_none() || doc_id().as_deref() == Some("");

    view! {
        <div class="editor-page">
            {move || {
                if welcome() {
                    view! {
                        <div class="welcome-screen">
                            <h1>"欢迎使用 NoteFlow"</h1>
                            <p>"选择一个文档开始编辑，或创建一个新文档。"</p>
                            <TemplatePicker/>
                        </div>
                    }.into_any()
                } else if doc_not_found() {
                    view! {
                        <div class="not-found">
                            <h1>"文档未找到"</h1>
                            <p>"该文档不存在或已被删除。"</p>
                        </div>
                    }.into_any()
                } else {
                    view! {
                        <div class="editor-layout">
                            <div class="editor-main">
                                <div class="editor-toolbar-row">
                                    <SearchPanel/>
                                    <TemplatePicker/>
                                </div>
                                <Editor/>
                            </div>
                            <aside class="editor-sidebar-right">
                                <TocPanel/>
                                <LinkGraph/>
                                <CommentPanel/>
                                <VersionHistory/>
                            </aside>
                        </div>
                    }.into_any()
                }
            }}
        </div>
    }
}
