// ============================================================
// 练习 e189: 手动提交 Action (action_dispatch)
//
// 核心知识点:
//   - action.dispatch(input) 手动提交（非表单方式）
//   - 不同输入触发不同的异步操作
//   - 追踪多个 dispatch 的结果
//   - action.input() 查看最近提交的输入
//
// 难度: ⭐⭐⭐
// ============================================================

use leptos::prelude::*;

// 模拟不同的异步操作
async fn process_task(task: &str) -> String {
    match task {
        "fetch" => "📦 数据获取完成：共 42 条记录".to_string(),
        "save" => "💾 数据保存成功！".to_string(),
        "delete" => "🗑️ 数据删除完成".to_string(),
        "refresh" => "🔄 数据已刷新".to_string(),
        _ => format!("⚙️ 未知任务 '{}' 已处理", task),
    }
}

#[component]
fn Exercise() -> impl IntoView {
    // TODO: 创建 Action，输入为任务名称字符串
    let action = Action::new(|input: &String| {
        let input = input.clone();
        async move { process_task(&input).await }
    });

    let tasks = vec![
        ("fetch", "获取数据"),
        ("save", "保存数据"),
        ("delete", "删除数据"),
        ("refresh", "刷新数据"),
    ];

    view! {
        <div>
            <p>"练习 189 — 手动提交 Action (action_dispatch)"</p>
            <p>"点击按钮手动 dispatch 不同的任务："</p>

            <div style="display: flex; gap: 8px; flex-wrap: wrap;">
                {tasks.into_iter().map(|(key, label)| {
                    let key = key.to_string();
                    view! {
                        <button
                            on:click=move |_| { action.dispatch(key.clone()); }
                            disabled=move || action.pending().get()
                        >
                            {label}
                        </button>
                    }
                }).collect::<Vec<_>>()}
            </div>

            <hr />
            <div>
                <p><strong>"最近提交:"</strong> {move || action.input().get().unwrap_or_default()}</p>
                <p>
                    <strong>"状态:"</strong>
                    {move || if action.pending().get() {
                        "⏳ 处理中...".to_string()
                    } else {
                        "✅ 空闲".to_string()
                    }}
                </p>
                <p>
                    <strong>"结果:"</strong>
                    {move || match action.value().get() {
                        None => "还没有结果".to_string(),
                        Some(v) => v,
                    }}
                </p>
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
//     async move { process_task(&input).await }
// });
//
// // 手动 dispatch
// <button on:click=move |_| action.dispatch("fetch".to_string())>
// ```
//
// ### 知识点
// - `action.dispatch(owned_input)` 可在任何事件处理器中调用，不限于表单
// - `action.input()` 返回 `ReadSignal<Option<Input>>`，记录最近一次提交的输入
// - 多次 dispatch 会按顺序执行，最新的结果覆盖之前的结果
// - Action 天然支持重复调用，每次 dispatch 都会运行异步闭包
//
// </details>
