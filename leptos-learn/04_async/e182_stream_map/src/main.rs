// ============================================================
// 练习 e182: Stream Map — 数据流转换
//
// 核心知识点:
//   - StreamExt::map() 对流的每个元素应用转换函数
//   - map 返回一个新的 Stream，原始流不变
//   - 常用于数据清洗、格式转换等场景
//
// 难度: ⭐⭐
// ============================================================

use futures::stream::{self, StreamExt};
use leptos::prelude::*;
use leptos::task::spawn_local;

#[component]
fn Exercise() -> impl IntoView {
    let items: RwSignal<Vec<i32>> = RwSignal::new(Vec::new());

    // TODO: 创建流并用 .map() 将每个元素乘以 2
    // 提示: stream::iter(1..=5).map(|x| x * 2)
    spawn_local(async move {
        let mut stream = stream::iter(1..=5).map(|x| x * 2);
        while let Some(value) = stream.next().await {
            items.update(|v| v.push(value));
            leptos::task::tick().await;
        }
    });

    view! {
        <div>
            <p>"练习 182 — Stream Map (stream_map)"</p>
            <p>"原始值 1..=5，map(x2) 后: " {move || {
                let v = items.get();
                if v.is_empty() {
                    "处理中...".to_string()
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
// let mut stream = stream::iter(1..=5).map(|x| x * 2);
// while let Some(value) = stream.next().await {
//     items.update(|v| v.push(value));
// }
// ```
// 输出: 2, 4, 6, 8, 10
//
// ### 知识点
// - `.map()` 接收闭包 `|item| transformed_item`，对每个元素进行变换
// - map 是惰性的：只在 poll/next 时执行转换
// - 原始流 `1,2,3,4,5` 保持不变，map 产生新流
//
// </details>
