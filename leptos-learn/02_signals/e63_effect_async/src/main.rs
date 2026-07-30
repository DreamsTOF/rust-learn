// ============================================================
// 练习 63: effect_async
//
// 目标: 在 Effect 中调用异步操作 (spawn_local)
//
// 难度: ⭐⭐⭐
// 核心知识点: spawn_local(async { ... })
//
// TODO: 补全 Effect 中的异步调用
// ============================================================

use leptos::prelude::*;
use leptos::task::spawn_local;

#[component]
fn Exercise() -> impl IntoView {
    let (count, set_count) = signal(0);
    let (msg, set_msg) = signal(String::new());

    // 在 Effect 中启动异步任务
    // 注意：Effect::new 的闭包本身不是 async，需要通过 spawn_local 启动异步块
    Effect::new(move || {
        let c = count.get();
        if c > 0 {
            set_msg.set(format!("正在加载 {}...", c));

            // spawn_local 将异步 Future 放入运行时执行
            spawn_local(async move {
                // 模拟异步操作，例如 API 请求
                // 真实场景中这里会是一个网络调用
                // 由于 leptos 的 effect 在 web 端默认使用本地执行器，
                // 此处用 wasm 友好的方式模拟延迟
                let result = simulate_async_work(c).await;
                set_msg.set(result);
            });
        } else {
            set_msg.set(String::new());
        }
    });

    view! {
        <p>"count: " {count}</p>
        <p>"msg: " {msg.clone()}</p>
        <button on:click=move |_| set_count.update(|n| *n += 1)>"+1"</button>
        <button on:click=move |_| set_count.set(0)>"重置"</button>
    }
}

/// 模拟异步操作
async fn simulate_async_work(n: i32) -> String {
    // 在 WASM 中，使用 wasm_bindgen_futures 或 leptos 的调度器
    // 此处简化为 yield 一次再返回
    leptos::task::tick().await;
    format!("异步结果: {}", n * 2)
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
// ### 代码
// ```rust
// use leptos::prelude::*;
// use leptos::task::spawn_local;
//
// Effect::new(move || {
//     let c = count.get();
//     if c > 0 {
//         spawn_local(async move {
//             let data = fetch_data(c).await;
//             set_data.set(data);
//         });
//     }
// });
//
// async fn fetch_data(n: i32) -> String {
//     // 网络请求等异步操作
//     format!("result {}", n)
// }
// ```
//
// ### 知识点
// - `Effect::new` 的闭包是同步的，内部通过 `spawn_local` 启动异步 Future
// - `spawn_local` 在当前线程的执行器上调度 Future（WASM 环境只能单线程）
// - 异步块内部访问信号要注意：`set_xxx` 可以在异步块中调用，但读取信号最好在 spawn 之前完成
// - 使用 `on_cleanup` 配合 `AbortController` / `CancellationToken` 可以取消正在进行的异步任务
//
// </details>
