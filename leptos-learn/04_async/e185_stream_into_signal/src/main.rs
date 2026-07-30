// ============================================================
// 练习 e185: Stream → Signal — 将 Stream 转为响应式信号
//
// 核心知识点:
//   - reactive_graph::traits::FromStream 将 Stream 转换为 Signal
//   - from_stream() 自动消费流并更新信号值
//   - ArcReadSignal<Option<T>> 信号类型
//   - 响应式系统自动追踪信号变化
//
// 难度: ⭐⭐⭐
// ============================================================

use futures::stream;
use leptos::prelude::*;
use leptos::task::spawn_local;

#[component]
fn Exercise() -> impl IntoView {
    // TODO: 使用 FromStream::from_stream() 将流转换为信号
    // 提示: FromStream trait 已包含在 leptos::prelude 中
    let signal: ArcReadSignal<Option<i32>> =
        leptos::reactive::traits::FromStream::from_stream(stream::iter(1..=5));

    // 模拟延时，让流有时间消费完
    spawn_local(async move {
        leptos::task::tick().await;
    });

    view! {
        <div>
            <p>"练习 185 — Stream → Signal (stream_into_signal)"</p>
            <p>"信号值: " {move || {
                match signal.get() {
                    None => "等待流数据...".to_string(),
                    Some(val) => format!("最新值: {}", val),
                }
            }}</p>
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
// ### 核心代码
// ```rust
// use leptos::reactive::traits::FromStream;
//
// let signal: ArcReadSignal<Option<i32>> =
//     FromStream::from_stream(stream::iter(1..=5));
// ```
//
// ### 知识点
// - `FromStream::from_stream(stream)` 自动消费流，每次更新信号为 `Some(value)`
// - 信号类型为 `ArcReadSignal<Option<T>>`，初始值为 `None`
// - 流结束后信号保持在最后一个值 `Some(last_value)`
// - `from_stream` 适用于 `Send + 'static` 的流(跨线程安全)
// - 若流不需要跨线程，可使用 `from_stream_unsync`（非 Send）
// - 这是将外部数据源（WebSocket、SSE、事件流）接入 Leptos 响应式系统的桥梁
//
// </details>
