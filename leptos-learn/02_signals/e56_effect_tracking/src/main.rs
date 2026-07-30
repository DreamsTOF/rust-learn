use leptos::prelude::*;

// ============================================================
// 练习 e56 — Effect 追踪：只追踪内部读取的信号
// 目标: 理解 Effect 只追踪在其闭包内被读取的信号
// 难度: ⭐⭐
// 核心知识点: Effect 只追踪内部读取的信号，不追踪未读的
// ============================================================

fn main() {
    mount_to_body(|| view! { <Exercise/> });
}

/// Effect 内只追踪被读取的信号，不追踪未读的
#[component]
fn Exercise() -> impl IntoView {
    let (a, set_a) = signal(0);
    let (b, set_b) = signal(0);

    // TODO: 创建 Effect，在其中只读取 a（不读取 b），观察 b 变化时 Effect 是否执行
    // 提示: 在 Effect 中只读取 a，然后分别点击两个按钮，观察控制台输出

    view! {
        <div>
            <h2>"练习 56 (effect_tracking)"</h2>
            <p>"Effect 只读取 a，不读取 b"</p>
            <p>"a: " {a}</p>
            <p>"b: " {b}</p>
            <button on:click=move |_| set_a(a() + 1)>"增加 a"</button>
            <button on:click=move |_| set_b(b() + 1)>"增加 b"</button>
            <p>"点击「增加 b」不会触发 Effect 重新执行"</p>
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
    let (a, set_a) = signal(0);
    let (b, set_b) = signal(0);

    // Effect 只读取 a，因此只追踪 a
    Effect::new(move |_| {
        leptos::logging::log!("Effect 执行: a = {}", a());
        // 注意: 这里没有读取 b，所以 b 的变化不会触发 Effect
    });

    view! {
        <div>
            <h2>"练习 56 (effect_tracking)"</h2>
            <p>"Effect 只读取 a，不读取 b"</p>
            <p>"a: " {a}</p>
            <p>"b: " {b}</p>
            <button on:click=move |_| set_a(a() + 1)>"增加 a"</button>
            <button on:click=move |_| set_b(b() + 1)>"增加 b"</button>
            <p>"点击「增加 b」不会触发 Effect 重新执行"</p>
        </div>
    }
}
```

### 知识点
- Effect 自动追踪在其闭包内被**读取**的信号
- 没有被读取的信号即使在其闭包作用域内（被 `move` 捕获）也不会被追踪
- 这避免了不必要的重复执行
- 追踪是动态的：如果 Effect 中有条件分支，只追踪当前分支读取的信号

</details>
*/
