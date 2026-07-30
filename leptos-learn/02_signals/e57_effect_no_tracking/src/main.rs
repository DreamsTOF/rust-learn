use leptos::prelude::*;

// ============================================================
// 练习 e57 — Effect 无追踪：未追踪的信号变化不会触发 Effect
// 目标: 理解 Effect 只在其追踪的信号变化时执行
// 难度: ⭐⭐
// 核心知识点: 信号在 Effect 外改变不会触发
// ============================================================

fn main() {
    mount_to_body(|| view! { <Exercise/> });
}

/// Effect 只在其追踪的信号变化时执行
#[component]
fn Exercise() -> impl IntoView {
    let (count, set_count) = signal(0);

    // TODO: 创建一个不读取任何信号的 Effect，观察点击按钮后它是否重新执行
    // 提示: Effect 中只输出固定文字，不读取 count
    // Effect::new(move |_| {
    //     leptos::logging::log!("Effect 执行了（但不会再次执行）");
    // });

    view! {
        <div>
            <h2>"练习 57 (effect_no_tracking)"</h2>
            <p>"Effect 没有追踪任何信号，所以只执行一次"</p>
            <p>"count: " {count}</p>
            <button on:click=move |_| set_count(count() + 1)>"增加 count"</button>
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

    // Effect 不读取任何信号，只执行一次
    Effect::new(move |_| {
        leptos::logging::log!("Effect 执行了（但不会再次执行）");
        // 没有读取任何信号，所以没有追踪任何依赖
        // count 的变化不会触发此 Effect
    });

    view! {
        <div>
            <h2>"练习 57 (effect_no_tracking)"</h2>
            <p>"Effect 没有追踪任何信号，所以只执行一次"</p>
            <p>"count: " {count}</p>
            <button on:click=move |_| set_count(count() + 1)>"增加 count"</button>
        </div>
    }
}
```

### 知识点
- 如果 Effect 内部没有读取任何信号，它只执行一次（下一次 tick）
- 信号变化不会再触发该 Effect，因为没有建立追踪关系
- 这是 Leptos 自动追踪机制的直接体现：不读取 = 不追踪 = 不触发
- 与 e56 同理：Effect 只追踪它真正读取的信号

</details>
*/
