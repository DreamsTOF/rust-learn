use leptos::prelude::*;

// ============================================================
// 练习 e11 — 条件 if 在 view 中
// 目标: 在 view! 中使用 if-else 条件渲染
// 难度: ⭐⭐⭐
// 核心知识点: { if cond { "A" } else { "B" } }
// ============================================================

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <Exercise/> });
}

/// TODO: 创建一个布尔信号和一个按钮
/// 点击按钮切换信号值，使用 if 表达式显示不同内容
#[component]
fn Exercise() -> impl IntoView {
    // TODO: 创建布尔信号 (true/false)
    let (show, set_show) = signal(false);

    view! {
        <div>
            <p>
                "当前状态: "
                // TODO: 使用 if 表达式根据 show 的值显示 "已激活" 或 "未激活"
                { if show() { "已激活" } else { "未激活" } }
            </p>
            // TODO: 添加按钮，点击时切换 show 的值
            <button on:click=move |_| set_show(!show())>
                "切换状态"
            </button>
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
    mount_to_body(|| view! { <Exercise/> });
}

#[component]
fn Exercise() -> impl IntoView {
    let (show, set_show) = signal(false);

    view! {
        <div>
            <p>
                "当前状态: "
                { if show() { "已激活" } else { "未激活" } }
            </p>
            <button on:click=move |_| set_show(!show())>
                "切换状态"
            </button>
        </div>
    }
}
```

### 知识点
- `{ if cond { "A" } else { "B" } }` 在 view! 中直接嵌入条件表达式，两分支必须类型一致
- `signal(bool)` 创建布尔响应式信号，`show()` 读取当前值
- `on:click=move |_| set_show(!show())` 按钮点击时取反信号值
- 条件表达式是 Rust 原生语法，Leptos 的 view! 宏支持直接嵌入

</details>
*/
