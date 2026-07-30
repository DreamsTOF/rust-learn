use leptos::prelude::*;

// ============================================================
// 练习 e58 — 多个 Effect：互不影响
// 目标: 创建 2 个独立 Effect 分别追踪不同信号
// 难度: ⭐⭐
// 核心知识点: 多个 Effect 互不影响
// ============================================================

fn main() {
    mount_to_body(|| view! { <Exercise/> });
}

/// 创建 2 个独立 Effect 分别追踪不同信号
#[component]
fn Exercise() -> impl IntoView {
    let (x, set_x) = signal(0);
    let (y, set_y) = signal(0);

    // TODO: 创建 2 个 Effect:
    //   Effect 1: 读取并打印 x
    //   Effect 2: 读取并打印 y
    //   观察各自只响应对应的信号变化

    view! {
        <div>
            <h2>"练习 58 (multi_effect)"</h2>
            <p>"两个 Effect 分别追踪 x 和 y，互不影响"</p>
            <p>"x: " {x}</p>
            <p>"y: " {y}</p>
            <button on:click=move |_| set_x(x() + 1)>"增加 x"</button>
            <button on:click=move |_| set_y(y() + 1)>"增加 y"</button>
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
    let (x, set_x) = signal(0);
    let (y, set_y) = signal(0);

    // Effect 1: 追踪 x
    Effect::new(move |_| {
        leptos::logging::log!("Effect 1 (追踪 x): x = {}", x());
    });

    // Effect 2: 追踪 y
    Effect::new(move |_| {
        leptos::logging::log!("Effect 2 (追踪 y): y = {}", y());
    });

    view! {
        <div>
            <h2>"练习 58 (multi_effect)"</h2>
            <p>"两个 Effect 分别追踪 x 和 y，互不影响"</p>
            <p>"x: " {x}</p>
            <p>"y: " {y}</p>
            <button on:click=move |_| set_x(x() + 1)>"增加 x"</button>
            <button on:click=move |_| set_y(y() + 1)>"增加 y"</button>
        </div>
    }
}
```

### 知识点
- 多个 Effect 可以共存，各自独立追踪自己的依赖
- Effect 1 追踪 x，只对 x 的变化响应
- Effect 2 追踪 y，只对 y 的变化响应
- 各 Effect 之间互不干扰，这是 Leptos 细粒度响应式的体现

</details>
*/
