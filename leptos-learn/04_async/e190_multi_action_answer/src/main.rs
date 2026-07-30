// ============================================================
// Exercise 190 - Multi Action
// ============================================================

use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    let (title, set_title) = signal(String::new());
    let (content, set_content) = signal(String::new());

    let save_draft = Action::new(|input: &(String, String)| {
        let (title, content) = input.clone();
        async move {
            format!("📝 草稿已保存：「{}」({} 字)", title, content.len())
        }
    });

    let publish = Action::new(|input: &(String, String)| {
        let (title, content) = input.clone();
        async move {
            format!("🚀 文章发布成功！「{}」\n内容长度: {} 字", title, content.len())
        }
    });

    view! {
        <div>
            <p>"练习 190 — 多个 Action (multi_action)"</p>
            <div>
                <div>
                    <label>"标题: "</label>
                    <input type="text" placeholder="文章标题"
                        on:input=move |ev| set_title(event_target_value(&ev))
                        prop:value=move || title.get()
                    />
                </div>
                <div>
                    <label>"内容: "</label>
                    <textarea placeholder="文章内容..."
                        on:input=move |ev| set_content(event_target_value(&ev))
                        prop:value=move || content.get()
                    />
                </div>
            </div>
            <div style="display: flex; gap: 8px; margin-top: 8px;">
                <button on:click=move |_| { save_draft.dispatch((title.get(), content.get())); }
                    disabled=move || save_draft.pending().get()>
                    {move || if save_draft.pending().get() { "保存中..." } else { "💾 保存草稿" }}
                </button>
                <button on:click=move |_| { publish.dispatch((title.get(), content.get())); }
                    disabled=move || publish.pending().get()>
                    {move || if publish.pending().get() { "发布中..." } else { "📤 正式发布" }}
                </button>
            </div>
            <hr />
            <div>
                <h3>"草稿状态:"</h3>
                {move || match save_draft.value().get() {
                    None => view! { <p>"尚未保存草稿"</p> }.into_any(),
                    Some(v) => view! { <pre style="color: #666;">{v}</pre> }.into_any(),
                }}
            </div>
            <div>
                <h3>"发布状态:"</h3>
                {move || match publish.value().get() {
                    None => view! { <p>"尚未发布"</p> }.into_any(),
                    Some(v) => view! { <pre style="color: green;">{v}</pre> }.into_any(),
                }}
            </div>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
