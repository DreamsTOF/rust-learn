use leptos::prelude::*;

// ============================================================
// 练习 e55 — Effect 响应式：信号改变 → Effect 重新执行
// 目标: 验证 Effect 在依赖的信号变化时自动重新执行
// 难度: ⭐
// 核心知识点: 信号改变 → Effect 重新执行
// ============================================================

fn main() {
    mount_to_body(|| view! { <Exercise/> });
}

/// Effect 响应信号变化，每次变化时打印
#[component]
fn Exercise() -> impl IntoView {
    let (count, set_count) = signal(0);

    // TODO: 创建 Effect 追踪 count，每次 count 变化时打印当前值
    // 提示: 点击按钮 → count 增加 → Effect 自动重新执行 → 控制台输出

    view! {
        <div>
            <h2>"练习 55 (effect_reactive)"</h2>
            <p>"Effect 追踪 count 的变化，每次变化都会重新执行"</p>
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

    // Effect 自动追踪 count，每次变化重新执行
    Effect::new(move |_| {
        leptos::logging::log!("count 变化: {}", count());
    });

    view! {
        <div>
            <h2>"练习 55 (effect_reactive)"</h2>
            <p>"Effect 追踪 count 的变化，每次变化都会重新执行"</p>
            <p>"count: " {count}</p>
            <button on:click=move |_| set_count(count() + 1)>"增加"</button>
        </div>
    }
}
```

### 知识点
- Effect 通过读取信号来自动追踪依赖
- 当依赖的信号变化时，Effect 自动重新执行
- 不需要手动声明依赖列表——依赖是自动追踪的
- 每次 Effect 重新执行时，都会用最新的信号值

</details>
*/
