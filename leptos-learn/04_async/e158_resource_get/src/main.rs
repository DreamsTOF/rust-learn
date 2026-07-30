// ============================================================
// 练习 158: resource_get — 获取数据
//
// 目标: 使用 .get() 获取 Resource 的数据值
//
// 难度: ⭐
// 核心知识点: Resource 的 .map() 返回 Option，None 表示加载中
//
// TODO: 按照注释提示补全代码
// ============================================================

use leptos::prelude::*;

/// 模拟异步加载（无额外延迟，但 Resource 首次加载时有短暂 loading 状态）
async fn fetch_score() -> String {
    "你的得分: 42".to_string()
}

#[component]
fn Exercise() -> impl IntoView {
    // === 步骤 1 ——————————————————————————————————————————
    // TODO: 创建 Resource
    let data = Resource::new(
        move || (),
        move |_| async move { fetch_score().await },
    );

    view! {
        <div>
            <p>"练习 158: resource_loading — 加载状态"</p>
            // TODO: 使用 .map() 判断加载状态
            //   - None → 显示 "加载中..."
            //   - Some(value) → 显示数据
            <p>
                {move || match data.map(|v| v.clone()) {
                    Some(score) => score.into_any(),
                    None => "正在加载...".into_any(),
                }}
            </p>
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
// ### 代码
// ```rust
// use leptos::prelude::*;
//
// async fn fetch_score() -> String {
//     "你的得分: 42".to_string()
// }
//
// #[component]
// fn Exercise() -> impl IntoView {
//     let data = Resource::new(
//         move || (),
//         move |_| async move { fetch_score().await },
//     );
//
//     view! {
//         <div>
//             <p>"练习 158: resource_loading — 加载状态"</p>
//             <p>
//                 {move || match data.map(|v| v.clone()) {
//                     Some(score) => score.into_any(),
//                     None => "正在加载...".into_any(),
//                 }}
//             </p>
//         </div>
//     }
// }
//
// fn main() {
//     mount_to_body(Exercise);
// }
// ```
//
// ### 知识点
// - `.map()` 返回 `Option<T>`：数据就绪为 `Some(value)`，加载中为 `None`
// - 通过匹配 `Option` 可以分别处理加载中和加载完成的状态
// - `.into_any()` 将不同类型统一为 `AnyView` 以便在 match 中使用
//
// </details>
