use leptos::prelude::*;

// ============================================================
// 练习 e54 — Effect 基础：创建第一个 Effect
// 目标: 使用 Effect::new 创建一个 Effect，在控制台输出
// 难度: ⭐
// 核心知识点: Effect::new(move || { ... })
// ============================================================

fn main() {
    mount_to_body(|| view! { <Exercise/> });
}

/// 创建第一个 Effect，在控制台输出信号值
#[component]
fn Exercise() -> impl IntoView {
    let (count, set_count) = signal(0);

    // TODO: 使用 Effect::new 创建一个 Effect，在控制台输出 count 的值
    // 提示: leptos::logging::log!("count: {}", count());

    view! {
        <div>
            <h2>"练习 54 (effect_new)"</h2>
            <p>"打开控制台 (F12) 查看 Effect 的输出"</p>
            <p>"count: " {count}</p>
            <button on:click=move |_| set_count(count() + 1)>"增加"</button>
        </div>
    }
}

/*
<details>
<summary>参考答案</summary>

```rust
use leptos::prelude::*;

fn main() {
    mount_to_body(|| view! { <Exercise/> });
}

#[component]
fn Exercise() -> impl IntoView {
    let (count, set_count) = signal(0);

    // 创建第一个 Effect
    Effect::new(move |_| {
        leptos::logging::log!("Effect 运行: count = {}", count());
    });

    view! {
        <div>
            <h2>"练习 54 (effect_new)"</h2>
            <p>"打开控制台 (F12) 查看 Effect 的输出"</p>
            <p>"count: " {count}</p>
            <button on:click=move |_| set_count(count() + 1)>"增加"</button>
        </div>
    }
}
```

### 知识点
- `Effect::new(move |_| { ... })` 创建一个 Effect
- Effect 在下一个 "tick" 执行（组件渲染之后）
- Effect 内部读取的信号会被自动追踪
- 参数 `_` 是上一次 Effect 返回的值，首次为 `None`
- 默认 Effect 不会在服务器端执行

</details>
*/
