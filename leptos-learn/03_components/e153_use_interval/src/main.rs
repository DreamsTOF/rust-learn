// ============================================================
// 练习 153: use_interval — 自定义 Interval Hook
//
// 目标: 封装 set_interval + on_cleanup，创建一个自动递增的
//       计时器 Hook，组件卸载时自动清理。
//
// 难度: ⭐⭐
// 核心知识点: set_interval、on_cleanup、资源清理
//
// TODO: 按照注释提示补全代码
// ============================================================

use leptos::prelude::*;
use std::time::Duration;

/// 自定义 Interval Hook
///
/// 每间隔 `duration` 调用一次 `callback`，并在组件卸载时自动清理
fn use_interval(callback: impl Fn() + 'static, duration: Duration) {
    // === 步骤 1 ——————————————————————————————————————————
    // 使用 set_interval 启动定时器，并用 on_cleanup 在组件卸载
    // 或 scope 销毁时清除定时器，防止内存泄漏。
    let handle = set_interval(callback, duration);

    // set_interval 返回的句柄由 leptos 的 reactive scope 管理，
    // 当 scope 被销毁时会自动清理。我们仍显式注册 on_cleanup
    // 以增强可读性和明确性。
    on_cleanup(move || {
        handle.clear();
    });
}

/// 返回一个每秒递增的 tick 信号
fn use_tick(interval_ms: u64) -> ReadSignal<u64> {
    let (tick, set_tick) = signal(0u64);

    use_interval(
        move || set_tick.update(|t| *t += 1),
        Duration::from_millis(interval_ms),
    );

    tick
}

#[component]
fn Exercise() -> impl IntoView {
    // === 步骤 2 ——————————————————————————————————————————
    // 使用 use_tick 创建一个每秒递增的计时器
    let tick = use_tick(1000);

    view! {
        <div>
            <h3>"练习 153: use_interval"</h3>
            <p>"已过秒数: " {tick}</p>
        </div>
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
// ### 核心思路
// 1. `set_interval` 启动定时器，`on_cleanup` 确保组件卸载时清除
// 2. 将信号更新放在回调中，实现响应式 tick 计数
//
// ### 关键代码
// ```rust
// fn use_interval(callback: impl Fn() + 'static, duration: Duration) {
//     set_interval(callback, duration);
//     on_cleanup(|| {});
// }
//
// fn use_tick(interval_ms: u64) -> ReadSignal<u64> {
//     let (tick, set_tick) = create_signal(0u64);
//     use_interval(move || set_tick.update(|t| *t += 1), Duration::from_millis(interval_ms));
//     tick
// }
// ```
//
// ### 知识点
// - `set_interval` 在 leptos 中受 reactive scope 管理
// - `on_cleanup` 在 scope 销毁时执行，适合清理定时器、事件监听等
// - 将副作用封装在 hook 内部，组件只需消费信号
//
// </details>
