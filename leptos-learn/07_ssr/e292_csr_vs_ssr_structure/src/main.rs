// ============================================================
// 练习 e292: CSR vs SSR 项目结构对比
//
// 核心知识点:
//   - CSR: 单文件 main.rs，mount_to_body 挂载
//   - SSR: main.rs(服务器) + lib.rs(路由) + app.rs(组件)
//   - 文件职责分离
//
// 难度: ⭐⭐ (关键位置有 TODO，约 50% 已补全)
// ============================================================

use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <div>
            <h2>"CSR vs SSR 项目结构"</h2>
            <div>
                <h3>"CSR (客户端渲染)"</h3>
                <ul>
                    <li>"src/main.rs" " — 所有代码（组件、逻辑、入口）"</li>
                </ul>
            </div>
            // TODO: 补全 SSR 结构说明
            <div>
                <h3>"SSR (服务端渲染)"</h3>
                <ul>
                    <li>"src/main.rs — 服务器入口，启动 Leptos 服务器"</li>
                    // TODO: 添加 lib.rs 列表项（应用入口，定义路由和 server fn）
                    <li>"src/lib.rs — 应用入口，定义路由和服务器函数"</li>
                    // TODO: 添加 app.rs 列表项（应用组件，定义页面 UI）
                    <li>"src/app.rs — 应用组件，定义页面 UI"</li>
                </ul>
            </div>
            // TODO: 添加说明段落：CSR 适合简单页面，SSR 适合 SEO 和完整应用
            <p>"CSR 适合简单交互页面，SSR 适合需要 SEO 和完整后端能力的应用。"</p>
        </div>
    }
}

fn main() {
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
//             <h2>"CSR vs SSR 项目结构"</h2>
//             <div>
//                 <h3>"CSR (客户端渲染)"</h3>
//                 <ul>
//                     <li>"src/main.rs" " — 所有代码（组件、逻辑、入口）"</li>
//                 </ul>
//             </div>
//             <div>
//                 <h3>"SSR (服务端渲染)"</h3>
//                 <ul>
//                     <li>"src/main.rs — 服务器入口，启动 Leptos 服务器"</li>
//                     <li>"src/lib.rs — 应用入口，定义路由和服务器函数"</li>
//                     <li>"src/app.rs — 应用组件，定义页面 UI"</li>
//                 </ul>
//             </div>
//             <p>"CSR 适合简单交互页面，SSR 适合需要 SEO 和完整后端能力的应用。"</p>
//         </div>
//     }
// }
//
// fn main() {
//     mount_to_body(Exercise);
// }
// </details>
