// ============================================================
// Exercise 153 - Answer: use_interval
// ============================================================

use leptos::prelude::*;
use std::time::Duration;

/// 自定义 Interval Hook
fn use_interval(callback: impl Fn() + 'static, duration: Duration) {
    let handle = set_interval(callback, duration);
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
