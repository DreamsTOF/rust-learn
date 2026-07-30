use leptos::ev;
use leptos::html::{button, div, h1, p};
use leptos::prelude::*;

// ============================================================
// 练习 e14 — 无宏构建器模式
// 目标: 使用构建器 API 替代 view! 宏创建元素
// 难度: ⭐⭐⭐
// 核心知识点: div().child("text").on(ev::click, ...)
// ============================================================

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <Exercise/> });
}

/// TODO: 使用 html::div() 等构建器替代 view! 宏
/// 链式调用 .child().on() 构建 DOM 树
/// 注意: 构建器返回的类型已实现 IntoView，无需调用 .build()
#[component]
fn Exercise() -> impl IntoView {
    let title = "构建器模式";
    let (count, set_count) = signal(0);

    // TODO: 使用构建器 API 创建包含标题、段落和按钮的 div
    //       所有子元素通过 .child() 链式添加，无需 .build()
    div()
        .child(h1().child(title))
        .child(p().child("使用构建器 API 创建，无需 view! 宏"))
        .child(button().child("点击: ").on(ev::click, move |_| set_count(count() + 1)))
}

/*
<details>
<summary>参考答案</summary>

```rust
use leptos::ev;
use leptos::html::{button, div, h1, p};
use leptos::prelude::*;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <Exercise/> });
}

#[component]
fn Exercise() -> impl IntoView {
    let (count, set_count) = signal(0);

    div()
        .child(h1().child("构建器模式"))
        .child(p().child("使用构建器 API 创建，无需 view! 宏"))
        .child(button().child("点击").on(ev::click, move |_| set_count(count() + 1)))
}
```

### 知识点
- `div().child("text").on(ev::click, handler)` 使用构建器 API
- 在此版 Leptos 中，构建器链直接实现 `IntoView`，无需 `.build()`
- `.child()` 支持嵌套构建器调用
- `.on(ev::click, handler)` 添加事件监听器
- 构建器 API 优点：类型安全、编译快、无需宏

</details>
*/
