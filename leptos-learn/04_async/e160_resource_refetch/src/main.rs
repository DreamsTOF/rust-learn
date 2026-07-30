// ============================================================
// 练习 160: resource_refetch — 手动刷新
//
// 目标: 使用 .refetch() 手动触发 Resource 重新加载
//
// 难度: ⭐⭐
// 核心知识点: Resource 的 .refetch() 方法
//
// TODO: 按照注释提示补全代码
// ============================================================

use leptos::prelude::*;

/// 模拟异步获取数据，每次调用返回不同的时间戳
async fn fetch_timestamp() -> String {
    format!("当前时间戳: {:?}", std::time::SystemTime::now())
}

#[component]
fn Exercise() -> impl IntoView {
    // === 步骤 1 ——————————————————————————————————————————
    // TODO: 创建 Resource
    let data = Resource::new(
        move || (),
        move |_| async move { fetch_timestamp().await },
    );

    view! {
        <div>
            <p>"练习 160: resource_refetch — 手动刷新"</p>
            // TODO: 显示 Resource 的数据
            <p>"数据: " {move || data.map(|v| v.clone())}</p>
            // TODO: 添加按钮，点击后调用 data.refetch() 手动刷新
            <button on:click=move |_| data.refetch()>
                "刷新数据"
            </button>
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
// async fn fetch_timestamp() -> String {
//     format!("当前时间戳: {:?}", std::time::SystemTime::now())
// }
//
// #[component]
// fn Exercise() -> impl IntoView {
//     let data = Resource::new(
//         move || (),
//         move |_| async move { fetch_timestamp().await },
//     );
//
//     view! {
//         <div>
//             <p>"练习 160: resource_refetch — 手动刷新"</p>
//             <p>"数据: " {move || data.map(|v| v.clone())}</p>
//             <button on:click=move |_| data.refetch()>
//                 "刷新数据"
//             </button>
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
// - `.refetch()` 手动触发 Resource 重新运行 fetcher
// - 即使 source 依赖值没有变化，`.refetch()` 也会强制重新加载
// - 常用于"刷新"按钮或下拉刷新等用户交互场景
// - `.refetch()` 与响应式依赖的区别：依赖自动触发 vs 手动触发
//
// </details>
