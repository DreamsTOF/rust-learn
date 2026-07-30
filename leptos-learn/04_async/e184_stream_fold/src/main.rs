// ============================================================
// 练习 e184: Stream Fold — 累积数据
//
// 核心知识点:
//   - StreamExt::fold() 将流的每个元素累积到初始值
//   - fold 返回 Future，需要 .await 获取最终结果
//   - 类似于迭代器的 .fold()，但流是异步的
//
// 难度: ⭐⭐⭐
// ============================================================

use futures::stream::{self, StreamExt};
use leptos::prelude::*;
use leptos::task::spawn_local;

#[component]
fn Exercise() -> impl IntoView {
    let result: RwSignal<Option<i32>> = RwSignal::new(None);

    // TODO: 使用 .fold() 计算 1..=5 的和
    // 提示: .fold(0, |acc, x| async move { acc + x })
    // 注意: fold 闭包返回 Future，所以需要 async move
    spawn_local(async move {
        let sum = stream::iter(1..=5)
            .fold(0, |acc, x| async move { acc + x })
            .await;
        result.set(Some(sum));
    });

    view! {
        <div>
            <p>"练习 184 — Stream Fold (stream_fold)"</p>
            <p>"1 + 2 + 3 + 4 + 5 = " {move || {
                match result.get() {
                    Some(sum) => sum.to_string(),
                    None => "计算中...".to_string(),
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
// let sum = stream::iter(1..=5)
//     .fold(0, |acc, x| async move { acc + x })
//     .await;
// // sum = 15
// ```
//
// ### 知识点
// - `.fold(init, |acc, item| async move { ... })` 累积流的所有元素
// - 与 Iterator::fold 不同，Stream::fold 的闭包返回 Future（异步）
// - 返回类型是 Future<Output = Acc>，需要 .await 获取结果
// - 当流结束时，fold 返回最终的累积值
// - 适用场景：求和、计数、拼接字符串等归约操作
//
// </details>
