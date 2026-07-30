// ============================================================
// 练习 e171: async fn — 在组件中使用 async 函数
//
// 目标: 定义一个 async 函数并在 Leptos 组件中调用它
//
// 难度: ⭐⭐
// 核心知识点: async fn、spawn_local、信号
// ============================================================

use leptos::prelude::*;
use leptos::task::spawn_local;

// TODO: 定义一个 async 函数，模拟异步获取数据
async fn fetch_message() -> String {
    "你好，async！".to_string()
}

#[component]
fn Exercise() -> impl IntoView {
    // TODO: 创建信号来存储异步结果
    let (message, set_message) = signal(String::new());

    // TODO: 使用 spawn_local 在组件中调用 async fn
    spawn_local(async move {
        let msg = fetch_message().await;
        set_message.set(msg);
    });

    view! {
        <div>
            <h2>"e171: async fn"</h2>
            <p>{message}</p>
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
// - `async fn` 返回 `impl Future<Output = T>`，通过 `.await` 执行
// - `spawn_local` 在当前线程上调度异步任务
// - 信号 (signal) 连接异步任务与响应式 UI
//
// </details>
