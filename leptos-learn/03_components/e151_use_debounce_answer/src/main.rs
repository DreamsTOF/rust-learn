// ============================================================
// Exercise 151 - Answer: use_debounce
// ============================================================

use leptos::prelude::*;
use std::time::Duration;

/// 自定义防抖 Hook
///
/// 接收初始值和延迟时间，返回：
///   - debounced: ReadSignal<T> — 防抖后的稳定值
///   - set_value:  WriteSignal<T> — 原始值的 setter
///   - is_pending: ReadSignal<bool> — 是否正在等待防抖
fn use_debounce<T>(initial: T, delay: Duration) -> (ReadSignal<T>, WriteSignal<T>, ReadSignal<bool>)
where
    T: Clone + Send + Sync + 'static,
{
    let (raw_value, set_raw_value) = signal(initial.clone());
    let (debounced, set_debounced) = signal(initial);
    let (is_pending, set_is_pending) = signal(false);

    Effect::new(move |_| {
        let current = raw_value();
        set_is_pending(true);

        let handle = set_timeout(
            move || {
                set_debounced.set(current);
                set_is_pending(false);
            },
            delay,
        );

        // 在 effect 重跑前取消上一个 set_timeout，实现防抖
        on_cleanup(move || {
            handle.clear();
        });
    });

    (debounced, set_raw_value, is_pending)
}

#[component]
fn Exercise() -> impl IntoView {
    let (debounced, set_value, is_pending) =
        use_debounce(String::new(), Duration::from_millis(500));

    view! {
        <div>
            <h3>"练习 151: use_debounce"</h3>
            <input
                type="text"
                placeholder="输入一些文本..."
                on:input=move |ev| set_value.set(event_target_value(&ev))
            />
            <p>"防抖值: " {debounced}</p>
            <p>"等待中: " {move || if is_pending() { "⏳ 是" } else { "✅ 否" }}</p>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
