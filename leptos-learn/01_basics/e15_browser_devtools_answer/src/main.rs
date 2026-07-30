// ============================================================
// 练习 e15: 浏览器开发者工具调试 — 参考答案
//
// 核心知识点:
//   - console_error_panic_hook::set_once() 将 Rust panic 输出到控制台
//   - tracing_wasm::set_as_global_default() 将 tracing 日志重定向到 WASM 控制台
//   - tracing::info!("消息") 在浏览器控制台输出 info 级别日志
//   - WASM 调试流程：tracing → tracing-wasm → console.log
// ============================================================

use leptos::prelude::*;

fn main() {
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default();
    tracing::info!("应用已启动");
    mount_to_body(|| view! { <Exercise/> });
}

#[component]
fn Exercise() -> impl IntoView {
    let (count, set_count) = signal(0);
    tracing::info!("初始计数: {}", count());

    view! {
        <div>
            <h2>"浏览器开发者工具调试"</h2>
            <p>"计数: " {count}</p>
            <button on:click=move |_| {
                set_count(count() + 1);
                tracing::info!("计数增加至: {}", count());
            }>
                "增加"
            </button>
            <p>
                <small>"打开浏览器开发者工具 (F12) → 控制台 查看日志"</small>
            </p>
        </div>
    }
}
