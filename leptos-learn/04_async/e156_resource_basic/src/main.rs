// ============================================================
// 练习 156: resource_basic — Resource::new() 基础
//
// 目标: 创建一个简单的 Resource，从异步函数加载数据
//
// 难度: ⭐⭐
// 核心知识点: Resource::new(source, fetcher)
//
// TODO: 按照注释提示补全代码
// ============================================================

use leptos::prelude::*;

/// 模拟异步数据获取函数
async fn fetch_greeting() -> String {
    "你好，Resource！".to_string()
}

#[component]
fn Exercise() -> impl IntoView {
    // === 步骤 1 ——————————————————————————————————————————
    // TODO: 使用 Resource::new 创建 Resource
    //   - source: || ()    (无依赖，只加载一次)
    //   - fetcher: |_| async { fetch_greeting().await }
    let data = Resource::new(
        move || (),
        move |_| async move { fetch_greeting().await },
    );

    view! {
        <div>
            <p>"练习 156: resource_basic — Resource::new() 基础"</p>
            // TODO: 使用 .map() 读取 Resource 的值并显示
            //   .map() 返回 Option<T>，当数据未就绪时为 None
            <p>"来自 Resource 的数据: " {move || data.map(|v| v.clone())}</p>
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
// async fn fetch_greeting() -> String {
//     "你好，Resource！".to_string()
// }
//
// #[component]
// fn Exercise() -> impl IntoView {
//     let data = Resource::new(
//         move || (),
//         move |_| async move { fetch_greeting().await },
//     );
//
//     view! {
//         <div>
//             <p>"练习 156: resource_basic — Resource::new() 基础"</p>
//             <p>"来自 Resource 的数据: " {move || data.map(|v| v.clone())}</p>
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
// - `Resource::new(source, fetcher)` 创建异步资源
// - source 闭包返回依赖值，fetcher 接收依赖值并返回 Future
// - 使用 `|| ()` 表示无依赖，资源只加载一次
// - `.map(|v| v.clone())` 同步、响应式地读取当前值
// - 返回 `Option<T>`，数据未就绪时为 `None`
//
// </details>
