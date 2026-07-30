// ============================================================
// 练习 157: resource_deps — 响应式依赖
//
// 目标: Resource 依赖信号变化自动重新加载
//
// 难度: ⭐⭐
// 核心知识点: Resource 的 source 响应式依赖
//
// TODO: 按照注释提示补全代码
// ============================================================

use leptos::prelude::*;

/// 模拟异步获取数据，接收一个参数 id
async fn fetch_item(id: i32) -> String {
    format!("Item #{} — 加载时间: {:?}", id, std::time::SystemTime::now())
}

#[component]
fn Exercise() -> impl IntoView {
    // === 步骤 1 ——————————————————————————————————————————
    // TODO: 创建信号 id，初始值为 1
    let (id, set_id) = signal(1);

    // === 步骤 2 ——————————————————————————————————————————
    // TODO: 创建 Resource，source 返回 id()，fetcher 使用 id 参数获取数据
    //   当 id 信号变化时，Resource 会自动重新加载
    let data = Resource::new(
        move || id(),
        move |id| async move { fetch_item(id).await },
    );

    view! {
        <div>
            <p>"练习 157: resource_deps — 响应式依赖"</p>
            <p>"当前 ID: " {id}</p>
            // TODO: 显示 Resource 的数据
            <p>"Resource 数据: " {move || data.map(|v| v.clone())}</p>
            // TODO: 添加按钮，点击后 id += 1，触发 Resource 自动重新加载
            <button on:click=move |_| set_id.update(|v| *v += 1)>
                "ID += 1"
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
// async fn fetch_item(id: i32) -> String {
//     format!("Item #{} — 加载时间: {:?}", id, std::time::SystemTime::now())
// }
//
// #[component]
// fn Exercise() -> impl IntoView {
//     let (id, set_id) = signal(1);
//
//     let data = Resource::new(
//         move || id(),
//         move |id| async move { fetch_item(id).await },
//     );
//
//     view! {
//         <div>
//             <p>"练习 157: resource_deps — 响应式依赖"</p>
//             <p>"当前 ID: " {id}</p>
//             <p>"Resource 数据: " {move || data.map(|v| v.clone())}</p>
//             <button on:click=move |_| set_id.update(|v| *v += 1)>
//                 "ID += 1"
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
// - source 闭包返回信号的值时，Resource 会自动追踪该信号
// - 当信号值变化时（与上次不同），Resource 自动重新运行 fetcher
// - 这是 Resource 响应式依赖的核心机制
// - source 返回的类型需要实现 `PartialEq` 以判断是否变化
//
// </details>
