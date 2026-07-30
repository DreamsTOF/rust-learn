// ============================================================
// 练习 e181: Stream 基础 — 创建基本 Stream
//
// 核心知识点:
//   - futures::stream::iter 从迭代器创建 Stream
//   - StreamExt::next() 异步获取流中的下一个元素
//   - spawn_local 在 WASM 环境中执行异步任务
//   - 将流的值存入信号以在视图中显示
//
// 难度: ⭐⭐⭐
// ============================================================

use futures::stream::{self, StreamExt};
use leptos::prelude::*;
use leptos::task::spawn_local;

#[component]
fn Exercise() -> impl IntoView {
    // 创建一个 RWSignal 来存储流中收集到的值
    let items: RwSignal<Vec<i32>> = RwSignal::new(Vec::new());

    // TODO: 使用 spawn_local 异步消费流
    // 提示: stream::iter(1..=5) 创建一个包含 1,2,3,4,5 的流
    //       使用 while let Some(value) = stream.next().await 来迭代
    spawn_local(async move {
        let mut stream = stream::iter(1..=5);
        while let Some(value) = stream.next().await {
            items.update(|v| v.push(value));
            leptos::task::tick().await; // 给响应式系统更新时间
        }
    });

    view! {
        <div>
            <p>"练习 181 — Stream 基础 (stream_basic)"</p>
            <p>"流中的元素: " {move || {
                let v = items.get();
                if v.is_empty() {
                    "等待中...".to_string()
                } else {
                    v.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(", ")
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
// use futures::stream::{self, StreamExt};
//
// let items: RwSignal<Vec<i32>> = RwSignal::new(Vec::new());
//
// spawn_local(async move {
//     let mut stream = stream::iter(1..=5);
//     while let Some(value) = stream.next().await {
//         items.update(|v| v.push(value));
//         leptos::task::tick().await;
//     }
// });
// ```
//
// ### 知识点
// - `stream::iter(iter)` 将任何迭代器转换为 Stream
// - `StreamExt::next()` 返回 `Option<Item>`，返回 `None` 表示流已结束
// - `spawn_local` 在 WASM 主线程上执行异步任务
// - 每次 `items.update()` 后调用 `tick()` 确保响应式系统及时更新
//
// </details>
