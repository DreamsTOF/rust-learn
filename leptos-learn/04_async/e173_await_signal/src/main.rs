// ============================================================
// 练习 e173: await_signal — 在异步任务中读写信号
//
// 目标: 在异步任务中多次读取和更新信号值
//
// 难度: ⭐⭐
// 核心知识点: 信号读写、异步任务循环
// ============================================================

use leptos::prelude::*;
use leptos::task::spawn_local;

#[component]
fn Exercise() -> impl IntoView {
    // TODO: 创建信号来追踪计数和状态
    let (count, set_count) = signal(0i32);
    let (status, set_status) = signal("准备就绪".to_string());

    // TODO: 在异步任务中循环更新信号值
    spawn_local(async move {
        for i in 1..=5 {
            set_count.set(i);
            set_status.set(format!("第 {i} 次更新"));
        }
    });

    view! {
        <div>
            <h2>"e173: await_signal"</h2>
            <p>"计数: " {count}</p>
            <p>{status}</p>
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
// - 异步任务中可以自由调用 `.set()` 更新信号
// - 信号更新会同步触发响应式系统重新渲染
// - 多个信号可以在同一个异步任务中独立更新
//
// </details>
