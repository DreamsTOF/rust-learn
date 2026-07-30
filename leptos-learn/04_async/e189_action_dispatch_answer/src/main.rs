// ============================================================
// Exercise 189 - Action Dispatch
// ============================================================

use leptos::prelude::*;

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
                <p><strong>"状态:"</strong> {move || if action.pending().get() { "⏳ 处理中..." } else { "✅ 空闲" }}</p>
                <p><strong>"结果:"</strong> {move || match action.value().get() {
                    None => "还没有结果".to_string(),
                    Some(v) => v,
                }}</p>
            </div>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
