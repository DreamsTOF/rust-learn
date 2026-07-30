// ============================================================
// 练习 e183: Stream Filter — 数据过滤
//
// 核心知识点:
//   - StreamExt::filter() 按条件过滤流中的元素
//   - filter 接收返回 bool 的闭包，保留 true 的元素
//   - 结合 map 可构建复杂的数据管道
//
// 难度: ⭐⭐
// ============================================================

use futures::future;
use futures::stream::{self, StreamExt};
use leptos::prelude::*;
use leptos::task::spawn_local;

#[component]
fn Exercise() -> impl IntoView {
    let items: RwSignal<Vec<i32>> = RwSignal::new(Vec::new());

    // TODO: 创建 1..=10 的流，用 .filter() 只保留偶数
    // 提示: .filter(|x| future::ready(*x % 2 == 0))
    spawn_local(async move {
        let mut stream = stream::iter(1..=10).filter(|x| future::ready(*x % 2 == 0));
        while let Some(value) = stream.next().await {
            items.update(|v| v.push(value));
            leptos::task::tick().await;
        }
    });

    view! {
        <div>
            <p>"练习 183 — Stream Filter (stream_filter)"</p>
            <p>"1..=10 中的偶数: " {move || {
                let v = items.get();
                if v.is_empty() {
                    "过滤中...".to_string()
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
// let mut stream = stream::iter(1..=10).filter(|x| future::ready(*x % 2 == 0));
// while let Some(value) = stream.next().await {
//     items.update(|v| v.push(value));
// }
// ```
// 输出: 2, 4, 6, 8, 10
//
// ### 知识点
// - `.filter()` 接收返回 `Future<Output = bool>` 的闭包
// - 使用 `future::ready(val)` 将同步值包装为立即完成的 Future
// - filter 是惰性适配器：迭代时才检查条件
// - 过滤条件使用 `*x` 解引用因为 stream::iter 产生 `&i32`（引用）
//
// </details>
