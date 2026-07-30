// ============================================================
// 练习 151: use_debounce — 输入防抖 Hook
//
// 目标: 实现一个通用 use_debounce hook，对输入值进行防抖处理，
//       延迟更新直到用户停止输入超过指定时间。
//
// 难度: ⭐⭐⭐
// 核心知识点: 防抖封装、set_timeout、on_cleanup
//
// TODO: 按照注释提示补全代码
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
    // === 步骤 1 ——————————————————————————————————————————
    // 创建三个信号:
    //   1. raw_value — 保存用户每次输入的原始值（未防抖）
    //   2. debounced — 对外暴露的防抖后值
    //   3. is_pending — 是否正在等待防抖计时器
    let (raw_value, set_raw_value) = signal(initial.clone());
    let (debounced, set_debounced) = signal(initial);
    let (is_pending, set_is_pending) = signal(false);

    // === 步骤 2 ——————————————————————————————————————————
    // 创建 effect: 每当 raw_value 变化时，清除上一个计时器，
    // 开启一个新的延迟计时器，到期后将最新值写入 debounced。
    //
    // 提示: 使用 set_timeout 设置延迟回调，on_cleanup 取消未到期的
    //       计时器（effect 重跑前会自动触发 on_cleanup）。
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
    // === 步骤 3 ——————————————————————————————————————————
    // 使用 use_debounce hook，初始值为空字符串，延迟 500ms
    let (debounced, set_value, is_pending) = use_debounce(String::new(), Duration::from_millis(500));

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

// ============================================================
// 参考答案 (思考后再看!)
// ============================================================
// <details>
// <summary>点击展开答案</summary>
//
// ### 核心思路
// 1. 创建两个信号：`raw_value` 保存每次输入的最新值，`debounced` 保存稳定后的值
// 2. 在 `create_effect` 中追踪 `raw_value()`，每次变化时调用 `set_timeout`
// 3. `on_cleanup` 确保上一个未到期的计时器被清除
// 4. 用户停止输入超过 delay 后，debounced 才会更新
//
// ### 关键代码
// ```rust
// fn use_debounce<T>(initial: T, delay: Duration) -> (ReadSignal<T>, WriteSignal<T>, ReadSignal<bool>)
// where
//     T: Clone + Send + Sync + 'static,
// {
//     let (raw_value, set_raw_value) = create_signal(initial.clone());
//     let (debounced, set_debounced) = create_signal(initial);
//     let (is_pending, set_is_pending) = create_signal(false);
//
//     create_effect(move |_| {
//         let current = raw_value();
//         set_is_pending(true);
//         set_timeout(move || {
//             set_debounced.set(current);
//             set_is_pending(false);
//         }, delay);
//         on_cleanup(|| {});
//     });
//
//     (debounced, set_raw_value, is_pending)
// }
// ```
//
// ### 知识点
// - 防抖模式：延迟执行直到"静默期"结束
// - leptos 的 `set_timeout` 受 reactive scope 管理，scope 销毁时自动清理
// - `on_cleanup` 在 effect 重跑前触发，确保前一个计时器被取消
// - 泛型约束 `Clone + Send + Sync + 'static` 保证值可以在异步边界传递
//
// </details>
