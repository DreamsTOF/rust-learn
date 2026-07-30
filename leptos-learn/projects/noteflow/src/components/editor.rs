use crate::state::AppState;
use leptos::prelude::*;

pub fn render_markdown(md: &str) -> String {
    use pulldown_cmark::{html, Options, Parser};
    let mut options = Options::all();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_HEADING_ATTRIBUTES);
    let parser = Parser::new_ext(md, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    // TODO: 练习 - 扩展 Markdown 渲染管道
    // 提示: 可以在此处添加额外的 HTML 后处理，如代码高亮、自定义容器等
    html_output
}

#[component]
pub fn Editor() -> impl IntoView {
    let state = use_context::<AppState>().expect("AppState not provided");
    let doc_content = RwSignal::new(String::new());
    let doc_title = RwSignal::new(String::new());

    // TODO: 练习 - 当激活的标签页变化时加载文档内容
    // 提示: 使用 Effect 监听 active_tab_id 变化，从 state.docs 中找到对应文档并更新 doc_content 和 doc_title
    {
        let state = state.clone();
        let doc_content = doc_content.clone();
        let doc_title = doc_title.clone();
        let _ = Effect::new(move |_| {
            let active_id = state.active_tab_id.get();
            if let Some(id) = active_id {
                let docs = state.docs.get();
                if let Some(doc) = docs.iter().find(|d| d.id == id) {
                    doc_title.set(doc.title.clone());
                    doc_content.set(doc.content.clone());
                }
            }
        });
    }

    let update_content = move |ev: leptos::ev::Event| {
        let new_content = event_target_value(&ev);
        doc_content.set(new_content.clone());
        let active_id = state.active_tab_id.get_untracked();
        if let Some(id) = active_id {
            state.docs.update(|docs| {
                for doc in docs.iter_mut() {
                    if doc.id == id {
                        doc.content = new_content.clone();
                        doc.updated_at = chrono::Utc::now().timestamp();
                    }
                }
            });
        }
    };

    let insert_text = move |before: &str, after: &str| {
        let current = doc_content.get_untracked();
        let new = format!("{}{}{}", before, current, after);
        doc_content.set(new);
    };

    let html_output = move || {
        let md = doc_content.get();
        render_markdown(&md)
    };

    view! {
        <div class="editor-container">
            <div class="editor-toolbar">
                <button
                    class="toolbar-btn"
                    title="加粗 (Ctrl+B)"
                    on:click=move |_| { insert_text("**", "**"); }
                >"B"</button>
                <button
                    class="toolbar-btn"
                    title="斜体 (Ctrl+I)"
                    on:click=move |_| { insert_text("*", "*"); }
                >"I"</button>
                <button
                    class="toolbar-btn"
                    title="标题"
                    on:click=move |_| { insert_text("## ", ""); }
                >"H"</button>
                <button
                    class="toolbar-btn"
                    title="列表"
                    on:click=move |_| { insert_text("- ", ""); }
                >"•"</button>
                <button
                    class="toolbar-btn"
                    title="引用"
                    on:click=move |_| { insert_text("> ", ""); }
                >"\""</button>
                <button
                    class="toolbar-btn"
                    title="代码"
                    on:click=move |_| { insert_text("```\n", "\n```"); }
                >"{"</button>
                <button
                    class="toolbar-btn"
                    title="链接"
                    on:click=move |_| { insert_text("[", "](url)"); }
                >"🔗"</button>
                <button
                    class="toolbar-btn"
                    title="图片"
                    on:click=move |_| { insert_text("![alt](", ")"); }
                >"🖼"</button>
            </div>
            <div class="editor-split">
                <div class="editor-pane">
                    <textarea
                        class="editor-textarea"
                        prop:value=doc_content
                        on:input=update_content
                        placeholder="在此输入 Markdown..."
                    ></textarea>
                </div>
                <div class="preview-pane">
                    <div class="preview-content" inner_html=html_output></div>
                </div>
            </div>
        </div>
    }
}
