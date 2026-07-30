// ============================================================
// 练习 e137: set_interval
//
// 目标: 使用 set_interval 实现定时轮询 / 动画
//
// 难度: ⭐⭐
// 核心知识点: set_interval
//
// TODO: 使用 set_interval 创建一个每秒递增的计数器
// ============================================================

use leptos::prelude::*;
use std::time::Duration;

#[component]
fn SetInterval() -> impl IntoView {
    let (count, set_count) = signal(0);

    set_interval(
        move || set_count.update(|n| *n += 1),
        Duration::from_secs(1),
    );

    view! {
        <div>
            <h2>"练习 e137: set_interval 定时器"</h2>
            <p>"每秒 +1"</p>
            <p style="font-size: 3rem; font-weight: bold;">{count}</p>
        </div>
    }
}

fn main() {
    mount_to_body(SetInterval);
}

// ============================================================
// 参考答案
// ============================================================
// <details>
// <summary>点击展开</summary>
//
// ```rust
// let (count, set_count) = signal(0);
// set_interval(move || set_count.update(|n| *n += 1), Duration::from_secs(1));
// ```
//
// `set_interval(cb, Duration)` 封装了 JS `setInterval`。
// 回调在 reactive tree 之外执行，但 WriteSignal 是 'static 的。
//
// </details>
