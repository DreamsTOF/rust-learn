// 练习 286: 日志与追踪 (tracing)
//
// 目标: 使用 tracing crate 在浏览器控制台中输出结构化日志。
// 需要先在 Cargo.toml 中添加 tracing-wasm.workspace = true
//
// 步骤:
// 1. 调用 tracing_wasm::set_as_global_default() 初始化 tracing 到 wasm
// 2. 创建一个 Exercise 组件，包含按钮和计数器
// 3. 点击按钮时使用 tracing::info! 记录日志
// 4. 当计数达到一定值时使用 tracing::warn! 发出警告

use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    let (count, set_count) = signal(0);

    view! {
        <div>
            <h2>"tracing 日志示例"</h2>
            <p>"点击次数: " {count}</p>
            <button on:click=move |_| {
                set_count.update(|n| *n += 1);
                // TODO: 使用 tracing::info! 记录点击事件，包含当前计数
                // 示例: tracing::info!("按钮被点击，当前次数: {}", count.get());

                // TODO: 当计数 >= 5 时，使用 tracing::warn! 发出警告
            }>"点击我"</button>
            <button on:click=move |_| {
                set_count.set(0);
                // TODO: 使用 tracing::info! 记录重置事件
            }>"重置"</button>
            <p>"打开浏览器控制台 (F12) 查看 tracing 日志"</p>
        </div>
    }
}

fn main() {
    // TODO: 初始化 tracing_wasm
    // tracing_wasm::set_as_global_default();
    mount_to_body(Exercise);
}
