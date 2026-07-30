// ============================================================
// 练习 e190: 多个 Action (multi_action)
//
// 核心知识点:
//   - 管理多个独立的 Action
//   - 不同 Action 处理不同的异步操作
//   - 每个 Action 有独立的 .pending() / .value() 状态
//   - 实际场景：保存草稿 + 正式提交
//
// 难度: ⭐⭐
// ============================================================

use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    let (title, set_title) = signal(String::new());
    let (content, set_content) = signal(String::new());

    // TODO: 创建两个 Action — save_draft 保存草稿，publish 正式发布
    let save_draft = Action::new(|input: &(String, String)| {
        let (title, content) = input.clone();
        async move {
            // 保存草稿（可以是本地存储或 API 调用）
            format!("📝 草稿已保存：「{}」({} 字)", title, content.len())
        }
    });

    let publish = Action::new(|input: &(String, String)| {
        let (title, content) = input.clone();
        async move {
            // 正式发布（模拟耗时较长的操作）
            format!("🚀 文章发布成功！「{}」\n内容长度: {} 字", title, content.len())
        }
    });

    view! {
        <div>
            <p>"练习 190 — 多个 Action (multi_action)"</p>

            <div>
                <div>
                    <label>"标题: "</label>
                    <input
                        type="text"
                        placeholder="文章标题"
                        on:input=move |ev| set_title(event_target_value(&ev))
                        prop:value=move || title.get()
                    />
                </div>
                <div>
                    <label>"内容: "</label>
                    <textarea
                        placeholder="文章内容..."
                        on:input=move |ev| set_content(event_target_value(&ev))
                        prop:value=move || content.get()
                    />
                </div>
            </div>

            <div style="display: flex; gap: 8px; margin-top: 8px;">
                <button
                    on:click=move |_| { save_draft.dispatch((title.get(), content.get())); }
                    disabled=move || save_draft.pending().get()
                >
                    {move || if save_draft.pending().get() { "保存中..." } else { "💾 保存草稿" }}
                </button>
                <button
                    on:click=move |_| { publish.dispatch((title.get(), content.get())); }
                    disabled=move || publish.pending().get()
                >
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

// ============================================================
// 参考答案 (思考后再看!)
// ============================================================
// <details>
// <summary>点击展开答案</summary>
//
// ### 核心代码
// ```rust
// let save_draft = Action::new(|input: &(String, String)| {
//     let (title, content) = input.clone();
//     async move {
//         format!("📝 草稿已保存：「{}」", title)
//     }
// });
//
// let publish = Action::new(|input: &(String, String)| {
//     let (title, content) = input.clone();
//     async move {
//         format!("🚀 文章发布成功！「{}」", title)
//     }
// });
// ```
//
// ### 知识点
// - 一个组件中可以创建多个 Action，每个管理独立的异步操作
// - 每个 Action 有独立的 `pending()`、`value()`、`input()` 信号
// - 多个 Action 之间互不干扰，可独立控制 UI 状态
// - 适合"保存草稿 + 正式提交"、"上传 + 删除"等多操作场景
//
// </details>
