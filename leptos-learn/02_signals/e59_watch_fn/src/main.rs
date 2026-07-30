// ============================================================
// 练习 59: watch_fn
//
// 目标: 使用 Effect::watch 显式追踪依赖，handler 只在依赖变化时运行
//
// 难度: ⭐⭐
// 核心知识点: watch(信号, move || ...)
//
// TODO: 补全 Effect::watch 调用
// ============================================================

use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    let (count, set_count) = signal(0);

    // 使用 Effect::watch 显式追踪 count
    // handler 接收 (当前值, 前一个值, 上一次返回值)
    // immediate: false 表示首次不运行，仅当依赖变化时触发
    Effect::watch(
        move || count.get(),
        move |count, prev_count, _| {
            println!("watch 触发: count={}, prev={:?}", count, prev_count);
        },
        false,
    );

    view! {
        <p>"count: " {count}</p>
        <button on:click=move |_| set_count.update(|n| *n += 1)>"+1"</button>
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
// Effect::watch(
//     move || count.get(),          // ① 显式指定依赖
//     move |val, prev, _| {         // ② handler 不追踪内部信号
//         println!("watch: {} -> {:?}", val, prev);
//     },
//     false,                        // ③ immediate: 首次不运行
// );
// ```
//
// ### 知识点
// - `Effect::watch` 的 handler **不会**自动追踪内部读取的信号，只有 dependency_fn 中的信号被追踪
// - 第三个参数 `immediate` 控制是否立即执行 handler
// - handler 的参数: (当前值, 上一次值, 上一次 handler 返回值)
// - 返回 `Effect` 类型，可调用 `.stop()` 停止
//
// </details>
