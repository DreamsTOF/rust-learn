// ============================================================
// 练习 e186: Action 基础 (action_basic)
//
// 核心知识点:
//   - Action::new() 创建异步 Action
//   - action.dispatch(input) 提交任务
//   - action.value() 获取结果信号
//   - action.pending() 获取加载状态信号
//
// 难度: ⭐⭐
// ============================================================

use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    // TODO: 使用 Action::new() 创建一个 Action
    // 提示: Action::new(|input: &String| async { ... })
    let action = Action::new(|input: &String| {
        let input = input.clone();
        async move {
            // Action 内部的异步逻辑：处理输入并返回结果
            format!("你好，{}！这是 Action 的处理结果。", input)
        }
    });

    let (name, set_name) = signal(String::new());

    view! {
        <div>
            <p>"练习 186 — Action 基础 (action_basic)"</p>
            <input
                type="text"
                placeholder="输入你的名字"
                on:input=move |ev| set_name(event_target_value(&ev))
                prop:value=name
            />
            <button
                on:click=move |_| { action.dispatch(name.get()); }
                disabled=move || action.pending().get()
            >
                {move || if action.pending().get() { "处理中..." } else { "提交" }}
            </button>
            <div>
                {move || action.value().get().map(|v| view! { <p><strong>{v}</strong></p> })}
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
//         format!("你好，{}！这是 Action 的处理结果。", input)
//     }
// });
// ```
//
// ### 知识点
// - `Action::new(f)` 接受闭包 `Fn(&Input) -> impl Future<Output = Output>`
// - `action.dispatch(owned_input)` 提交输入，触发异步任务
// - `action.value()` → `ReadSignal<Option<Output>>`，最新结果
// - `action.pending()` → `ReadSignal<bool>`，是否正在执行
// - Action 自动管理输入 → 异步处理 → 输出 的完整生命周期
//
// </details>
