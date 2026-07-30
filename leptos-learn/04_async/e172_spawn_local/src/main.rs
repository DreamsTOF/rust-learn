// ============================================================
// 练习 e172: spawn_local — 在事件处理中启动异步任务
//
// 目标: 在按钮点击事件中使用 spawn_local 执行异步操作
//
// 难度: ⭐⭐
// 核心知识点: spawn_local、事件处理、信号
// ============================================================

use leptos::prelude::*;
use leptos::task::spawn_local;

// TODO: 定义一个 async 函数，模拟异步工作
async fn do_work() -> String {
    "任务完成！".to_string()
}

#[component]
fn Exercise() -> impl IntoView {
    // TODO: 创建信号来追踪任务状态
    let (status, set_status) = signal("点击按钮启动任务".to_string());

    view! {
        <div>
            <h2>"e172: spawn_local"</h2>
            <p>{status}</p>
            // TODO: 点击按钮时使用 spawn_local 启动异步任务
            <button on:click=move |_| {
                set_status.set("执行中...".to_string());
                spawn_local(async move {
                    let result = do_work().await;
                    set_status.set(result);
                });
            }>"启动任务"</button>
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
// ### 知识点
// - `spawn_local` 可在事件处理器中启动异步任务
// - 信号更新会自动触发 UI 重新渲染
// - 异步任务完成后通过信号将结果传回 UI
//
// </details>
