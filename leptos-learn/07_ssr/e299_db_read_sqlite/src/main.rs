// ============================================================
// 练习 e299: 从 SQLite 读取数据 (DB Read)
//
// 核心知识点:
//   - #[server] 函数从数据库查询数据并返回行
//   - 使用 Action 调用服务端函数获取数据
//   - 客户端展示数据列表
//   - sqlx 查询模式介绍
//
// 难度: ⭐⭐ (关键 TODO 已标记)
// ============================================================

use leptos::prelude::*;
use leptos::prelude::ServerFnError;

// ⭐⭐ TODO: 定义 Item 数据模型
// 提示: pub struct Item { pub id: i32, pub name: String, pub description: String }
// 在真实应用中会通过 sqlx::FromRow 派生宏与数据库表对应

// ⭐⭐ TODO: 使用 #[server(GetItems)] 定义 get_items 服务端函数
// 不接受参数，返回 Result<Vec<Item>, ServerFnError>
// 提示: 函数体中，模拟数据；真实应用中调用 sqlx::query_as

#[component]
fn Exercise() -> impl IntoView {
    // ⭐⭐ TODO: 创建 Action 包装 get_items
    // 提示: Action::new(|_: &()| async move { get_items().await.unwrap_or_default() })

    view! {
        <div>
            <h2>"练习 299: SQLite 数据读取"</h2>
            <p>"点击按钮从服务端获取项目列表。"</p>

            // ⭐⭐ TODO: 添加按钮，点击时触发 load_action.dispatch(())
            // 提示: pending 时显示 "加载中..."，否则显示 "获取项目列表"
            // 提示: disabled=move || load_action.pending().get()

            // ⭐⭐ TODO: 使用 action.value() 显示加载结果
            // 提示: 使用 .map() 处理 Option<Vec<Item>>，遍历显示每个项目的 name 和 description
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}

// <details>
// <summary>点击展开答案</summary>
//
// ```rust
// use leptos::prelude::*;
// use leptos::prelude::ServerFnError;
//
// #[derive(Debug, Clone)]
// pub struct Item {
//     pub id: i32,
//     pub name: String,
//     pub description: String,
// }
//
// #[server(GetItems)]
// pub async fn get_items() -> Result<Vec<Item>, ServerFnError> {
//     // 真实 SSR 应用中会使用 sqlx:
//     //   let pool = /* 获取连接池 */;
//     //   let items = sqlx::query_as::<_, Item>("SELECT id, name, description FROM items")
//     //       .fetch_all(pool)
//     //       .await
//     //       .map_err(|e| ServerFnError::ServerError(e.to_string()))?;
//     //   Ok(items)
//     //
//     Ok(vec![
//         Item { id: 1, name: "学习 Leptos".into(), description: "掌握 Leptos 0.8 nightly API".into() },
//         Item { id: 2, name: "编写练习".into(), description: "完成 SSR 相关练习".into() },
//         Item { id: 3, name: "部署上线".into(), description: "配置 VPS 环境".into() },
//     ])
// }
//
// #[component]
// fn Exercise() -> impl IntoView {
//     let load_action = Action::new(|_: &()| async move {
//         get_items().await.unwrap_or_default()
//     });
//
//     view! {
//         <div style="padding: 1.5rem; font-family: system-ui, sans-serif; max-width: 40rem; margin: 0 auto;">
//             <h2>"练习 299: SQLite 数据读取"</h2>
//             <p>"点击按钮从服务端获取项目列表。"</p>
//             <button
//                 on:click=move |_| { load_action.dispatch(()); }
//                 disabled=move || load_action.pending().get()
//             >
//                 {move || if load_action.pending().get() { "加载中..." } else { "获取项目列表" }}
//             </button>
//             <div style="margin-top: 1rem;">
//                 {move || load_action.value().get().map(|items| {
//                     view! {
//                         <ul style="list-style: none; padding: 0;">
//                             {items.iter().map(|item| {
//                                 view! {
//                                     <li style="padding: 0.5rem; border-bottom: 1px solid #e2e8f0;">
//                                         <strong>{&item.name}</strong>
//                                         " — "
//                                         {&item.description}
//                                     </li>
//                                 }
//                             }).collect::<Vec<_>>()}
//                         </ul>
//                     }
//                 })}
//             </div>
//         </div>
//     }
// }
//
// fn main() {
//     mount_to_body(Exercise);
// }
// ```
// </details>
