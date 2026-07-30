// ============================================================
// 练习 e166: 局部 Resource (resource_local)
//
// 核心知识点:
//   - LocalResource: 仅在客户端组件内加载的资源
//   - 与 Resource 的区别：不参与 SSR 序列化
//   - 使用 .get() 读取 Option 值
//
// 难度: ⭐⭐⭐
// ============================================================

use leptos::prelude::*;

// 模拟异步数据获取（仅客户端执行）
async fn fetch_greeting() -> String {
    "你好，世界！这是 LocalResource 加载的数据。".to_string()
}

#[component]
fn Exercise() -> impl IntoView {
    // TODO: 使用 LocalResource::new 创建一个局部资源
    // 提示: LocalResource::new(|| async move { ... }) 只需要一个 fetcher 闭包
    let greeting = LocalResource::new(|| async move {
        fetch_greeting().await
    });

    view! {
        <div>
            <h2>"LocalResource 示例"</h2>
            // TODO: 使用 greeting.get() 读取资源值
            // 提示: .get() 返回 Option<String>
            {match greeting.get() {
                None => view! { <p>"加载中..."</p> }.into_any(),
                Some(data) => view! { <p>{data}</p> }.into_any(),
            }}
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
//     "你好，世界！这是 LocalResource 加载的数据。".to_string()
// }
//
// #[component]
// fn Exercise() -> impl IntoView {
//     let greeting = LocalResource::new(|| async move {
//         fetch_greeting().await
//     });
//
//     view! {
//         <div>
//             <h2>"LocalResource 示例"</h2>
//             {match greeting.get() {
//                 None => view! { <p>"加载中..."</p> }.into_any(),
//                 Some(data) => view! { <p>{data}</p> }.into_any(),
//             }}
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
// - LocalResource 只在客户端执行，不参与 SSR 序列化
// - LocalResource::new(fetcher) 只需一个闭包，无需依赖参数
// - .get() 返回 Option<T>，初始为 None，加载完成后为 Some
// - 内部会自动调用 any_spawner::Executor::tick() 等待一个微任务
//   以避免水合时的时间差问题
// - 如果需要依赖响应式信号，可在闭包内捕获信号值
//
// </details>
