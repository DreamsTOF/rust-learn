// ============================================================
// 练习 e291: cargo-leptos 项目结构
//
// 核心知识点:
//   - cargo-leptos: Leptos SSR 官方项目脚手架
//   - SSR 项目文件结构: main.rs / lib.rs / app.rs
//   - 运行命令: cargo leptos watch
//
// 难度: ⭐ (填空题 — 每行都有 TODO 指引)
// ============================================================

use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <div>
            // TODO: 添加 <h2> 标题 "cargo-leptos SSR 项目结构"
            <h2>"cargo-leptos SSR 项目结构"</h2>
            // TODO: 添加 <p> 段落，说明 cargo-leptos 的用途
            <p>"cargo-leptos 是 Leptos 官方推荐的 SSR 项目脚手架工具。"</p>
            // TODO: 用 <ul> 和 <li> 展示 SSR 项目的核心文件列表
            <ul>
                // TODO: 补全以下列表项，在冒号后填写文件说明
                <li>"src/main.rs — 服务器入口，启动 Leptos 服务器"</li>
                <li>"src/lib.rs — 应用入口，定义路由和服务器函数"</li>
                <li>"src/app.rs — 应用组件，定义页面 UI"</li>
            </ul>
            // TODO: 添加 <p> 段落显示运行命令
            <p>"运行: " <code>"cargo leptos watch"</code></p>
        </div>
    }
}

fn main() {
    // TODO: 使用 mount_to_body 挂载 Exercise 组件
    mount_to_body(Exercise);
}

// <details>
// 参考答案:
//
// use leptos::prelude::*;
//
// #[component]
// fn Exercise() -> impl IntoView {
//     view! {
//         <div>
//             <h2>"cargo-leptos SSR 项目结构"</h2>
//             <p>"cargo-leptos 是 Leptos 官方推荐的 SSR 项目脚手架工具。"</p>
//             <ul>
//                 <li>"src/main.rs — 服务器入口，启动 Leptos 服务器"</li>
//                 <li>"src/lib.rs — 应用入口，定义路由和服务器函数"</li>
//                 <li>"src/app.rs — 应用组件，定义页面 UI"</li>
//             </ul>
//             <p>"运行: " <code>"cargo leptos watch"</code></p>
//         </div>
//     }
// }
//
// fn main() {
//     mount_to_body(Exercise);
// }
// </details>
