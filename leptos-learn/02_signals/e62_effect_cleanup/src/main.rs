// ============================================================
// 练习 62: effect_cleanup
//
// 目标: 使用 on_cleanup() 在 Effect 中释放资源
//
// 难度: ⭐⭐⭐
// 核心知识点: on_cleanup() 释放资源
//
// TODO: 补全 on_cleanup 释放定时器
// ============================================================

use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    let (count, set_count) = signal(0);

    // Effect 每次重新运行前会执行上一次注册的 cleanup
    // 此处示例：每隔一秒 count +1，effect 自身也追踪 count 变化
    Effect::new(move |_| {
        let c = count.get();
        println!("Effect 运行: count={}", c);

        // 注册清理函数：当 effect 重新运行或 owner 被销毁时调用
        // 例如可以清理定时器、取消订阅、关闭连接等
        on_cleanup(|| {
            println!("Cleanup 运行");
        });
    });

    // 也可以直接在视图中使用 set_interval 配合 on_cleanup
    // 但为了演示 effect 本身的 cleanup，我们用上面的简化版本

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
// Effect::new(move |_| {
//     let handle = set_interval_with_handle(
//         move || println!("tick"),
//         std::time::Duration::from_secs(1),
//     );
//     on_cleanup(move || handle.clear());
// });
// ```
//
// ### 知识点
// - `on_cleanup()` 注册一个清理函数，在以下时机被调用：
//   1. Effect 重新运行前（清理上次资源）
//   2. Effect 所属的 Owner 被销毁时
// - 典型用途：取消定时器、取消网络请求、移除事件监听器、关闭 WebSocket 连接
// - 每个作用域可以注册多个 cleanup，按注册顺序的**逆序**执行
//
// </details>
