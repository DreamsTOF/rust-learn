// ============================================================
// 练习 60: watch_vs_effect
//
// 目标: 对比 Effect::new（自动追踪）与 Effect::watch（显式指定依赖）的差异
//
// 难度: ⭐⭐
// 核心知识点: watch 显式指定依赖 vs Effect
//
// TODO: 补全两种方式
// ============================================================

use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    let (a, set_a) = signal(0);
    let (b, set_b) = signal(0);

    // Effect::new — 自动追踪内部所有信号
    // 只要 a 或 b 发生变化都会触发
    Effect::new(move || {
        println!("Effect::new 触发: a={}, b={}", a.read(), b.read());
    });

    // Effect::watch — 只追踪 dependency_fn 中的信号
    // 只有 a 变化时触发，b 的变化不会触发
    Effect::watch(
        move || a.get(),
        move |val, _prev, _| {
            println!("Effect::watch 触发: a={}", val);
        },
        false,
    );

    view! {
        <p>"a: " {a} " | b: " {b}</p>
        <button on:click=move |_| set_a.update(|n| *n += 1)>"a +1"</button>
        <button on:click=move |_| set_b.update(|n| *n += 1)>"b +1"</button>
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
// // Effect::new — 自动追踪
// Effect::new(move || {
//     println!("a={}, b={}", a.read(), b.read());
// });
//
// // Effect::watch — 显式指定依赖
// Effect::watch(
//     move || a.get(),
//     move |val, _prev, _| { println!("a={}", val); },
//     false,
// );
// ```
//
// ### 知识点
// | 特性 | Effect::new | Effect::watch |
// |------|------------|---------------|
// | 依赖追踪 | 自动追踪闭包内所有信号 | 只追踪 dependency_fn 中的信号 |
// | handler 内信号 | 也会被追踪 | **不会**被追踪 |
// | 参数 | `Option<T>` (上一次返回值) | `(&D, Option<&D>, Option<T>)` |
// | 适用场景 | 简单副作用 | 精确控制依赖，避免不必要的触发 |
//
// </details>
