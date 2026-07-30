use leptos::prelude::*;

// ============================================================
// 练习 e13 — 索引/方法调用
// 目标: 在 view! 中调用 Vec 的方法和索引
// 难度: ⭐⭐
// 核心知识点: { items.len() }, { items[0] }
// ============================================================

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <Exercise/> });
}

/// 使用 Vec 存储数据，在 view! 中调用 len() 和索引访问
#[component]
fn Exercise() -> impl IntoView {
    // 创建一个 Vec 作为数据源
    let items = vec!["Rust", "Leptos", "WASM"];

    view! {
        <div>
            <h2>"编程语言列表"</h2>
            <p>
                "共有 "
                // TODO: 调用 items.len() 获取长度
                { items.len() }
                " 门语言"
            </p>
            <p>
                "第一门语言: "
                // TODO: 使用 items[0] 索引访问第一个元素
                { items[0] }
            </p>
            <p>
                "第二门语言: "
                // TODO: 使用 items[1] 索引访问第二个元素
                { items[1] }
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
    mount_to_body(|| view! { <Exercise/> });
}

#[component]
fn Exercise() -> impl IntoView {
    let items = vec!["Rust", "Leptos", "WASM"];

    view! {
        <div>
            <h2>"编程语言列表"</h2>
            <p>"共有 " { items.len() } " 门语言"</p>
            <p>"第一门语言: " { items[0] }</p>
            <p>"第二门语言: " { items[1] }</p>
        </div>
    }
}
```

### 知识点
- `{ items.len() }` 在 view! 中调用 Vec 的 len() 方法
- `{ items[0] }` 使用索引语法访问 Vec 元素
- `{ items[1] }` 索引从 0 开始
- 普通 Rust 表达式（非响应式）在 view! 中可直接嵌入，值在渲染时确定

</details>
*/
