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
    tracing::info!("应用已启动");

    mount_to_body(|| view! { <Exercise/> });
}

/// 在按钮点击时输出调试日志，演示 WASM 调试技巧
#[component]
fn Exercise() -> impl IntoView {
    let (count, set_count) = signal(0);

    // TODO: 使用 tracing::info! 输出当前计数
    tracing::info!("初始计数: {}", count());

    view! {
        <div>
            <h2>"浏览器开发者工具调试"</h2>
            <p>"计数: " {count}</p>
            <button on:click=move |_| {
                set_count(count() + 1);
                // TODO: 点击时输出调试信息
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

/*
<details>
<summary>参考答案</summary>

```rust
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
```

### 知识点
- `console_error_panic_hook::set_once()` 将 Rust panic 信息输出到控制台
- `tracing_wasm::set_as_global_default()` 将 tracing 日志重定向到 WASM 控制台
- `tracing::info!("消息")` 在浏览器控制台输出 info 级别日志
- WASM 调试流程：`tracing` → `tracing-wasm` → `console.log`
- 打开浏览器 F12 → 控制台 即可查看 Rust 侧输出的日志

</details>
*/
