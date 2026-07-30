// ============================================================
// 练习 e188: Action 加载状态 (action_pending)
//
// 核心知识点:
//   - action.pending() 响应式加载状态信号
//   - 提交中禁用按钮 / 显示加载指示器
//   - 异步任务完成后自动更新状态
//   - 提交时禁用按钮防止重复提交
//
// 难度: ⭐⭐
// ============================================================

use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    let (input, set_input) = signal(String::new());

    // TODO: 创建一个 Action，模拟异步处理
    let action = Action::new(|input: &String| {
        let input = input.clone();
        async move {
            // 模拟异步处理（真实场景中这里可能是 API 调用）
            format!("处理完成！输入内容长度: {} 字符", input.len())
        }
    });

    view! {
        <div>
            <p>"练习 188 — Action 加载状态 (action_pending)"</p>
            <input
                type="text"
                placeholder="输入一些文字"
                on:input=move |ev| set_input(event_target_value(&ev))
                prop:value=move || input.get()
            />
            <button
                on:click=move |_| { action.dispatch(input.get()); }
                disabled=move || action.pending().get()
            >
                {move || if action.pending().get() { "处理中..." } else { "开始处理" }}
            </button>

            <div>
                // 提交中禁用按钮（pending() 为 true 时按钮不可点击）
                {move || if action.pending().get() {
                    view! { <p style="color: orange;">"⏳ 正在处理，请勿重复提交..."</p> }.into_any()
                } else {
                    view! {}.into_any()
                }}

                // 显示处理结果
                {move || action.value().get().map(|v| view! { <p style="color: green;"><strong>{v}</strong></p> })}
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
// let action = Action::new(|input: &String| {
//     let input = input.clone();
//     async move {
//         format!("处理完成！输入内容长度: {} 字符", input.len())
//     }
// });
//
// // 使用 pending() 控制按钮状态
// <button disabled=move || action.pending().get()>
// {move || if action.pending().get() { "处理中..." } else { "开始处理" }}
// </button>
// ```
//
// ### 知识点
// - `action.pending()` 在异步任务开始时自动变为 `true`，完成后变回 `false`
// - 利用 `pending()` 可禁用按钮避免重复提交
// - 在表单场景中，`pending()` 可用于显示加载动画或进度条
// - 即使异步任务很快完成，`pending()` 信号也会短暂为 `true`
//
// </details>
