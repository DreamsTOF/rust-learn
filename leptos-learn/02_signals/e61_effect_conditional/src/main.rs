// ============================================================
// 练习 61: effect_conditional
//
// 目标: 理解 Effect 中条件分支对信号追踪的影响
//
// 难度: ⭐⭐
// 核心知识点: Effect 中的条件分支 if count() > 0 { ... } 条件追踪
//
// TODO: 补全 Effect::new 中的条件逻辑
// ============================================================

use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    let (count, set_count) = signal(0);
    let (text, set_text) = signal(String::new());

    // 当 count > 0 时，追踪 text 的长度
    // 当 count <= 0 时，不追踪 text，修改 text 不会触发 effect
    Effect::new(move || {
        let c = count.get();
        if c > 0 {
            println!("count={}, text len={}", c, text.read().len());
        } else {
            println!("count={}, 未追踪 text", c);
        }
    });

    view! {
        <p>"count: " {count}</p>
        <p>"text: " {text.clone()}</p>
        <button on:click=move |_| set_count.update(|n| { *n += 1; })>"count +1"</button>
        <button on:click=move |_| set_count.update(|n| { if *n > 0 { *n -= 1; } })>"count -1"</button>
        <input
            prop:value=move || text.get()
            on:input=move |e| set_text.set(event_target_value(&e))
        />
    }
}

fn main() {
    mount_to_body(Exercise);
}

// ============================================================
// 参考答案 (思考后再看!)
// ============================================================
// <details>
// <summary>点击展开答案</summary>
//
// ### 代码
// ```rust
// Effect::new(move || {
//     let c = count.get();
//     if c > 0 {
//         // 仅在 c > 0 时追踪 text
//         println!("count={}, text len={}", c, text.read().len());
//     }
// });
// ```
//
// ### 知识点
// - Leptos 的依赖追踪是**运行时动态**的：只追踪实际执行路径上访问的信号
// - 当 `count <= 0` 时，`text` 未被访问，修改 `text` **不会**触发 effect 重新运行
// - 这称为"条件追踪"或"动态依赖" — 依赖集可以随时间变化
// - 这是性能优势：避免不必要的计算
//
// </details>
