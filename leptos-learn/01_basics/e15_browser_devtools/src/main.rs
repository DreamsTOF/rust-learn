use leptos::prelude::*;

// ============================================================
// 练习 e15 — 调试：浏览器开发者工具
// 目标: 使用 logging 在浏览器控制台输出调试信息
// 难度: ⭐⭐
// 核心知识点: WASM 调试、console.log、tracing-wasm
// ============================================================

fn main() {
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default();
 
    // TODO: 使用 tracing::info! 在浏览器控制台输出启动信息
 
    mount_to_body(|| view! { <Exercise/> });
}

/// 在按钮点击时输出调试日志，演示 WASM 调试技巧
#[component]
fn Exercise() -> impl IntoView {
    let (count, set_count) = signal(0);
 
    // TODO: 使用 tracing::info! 输出当前计数
 
    view! {
        <div>
            <h2>"浏览器开发者工具调试"</h2>
            <p>"计数: " {count}</p>
            <button on:click=move |_| {
                set_count.set(count.get() + 1);
                // TODO: 点击时输出调试信息
            }>
                "增加"
            </button>
            <p>
                <small>"打开浏览器开发者工具 (F12) → 控制台 查看日志"</small>
            </p>
        </div>
    }
}
