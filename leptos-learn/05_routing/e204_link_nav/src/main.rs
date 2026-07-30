// ============================================================
// 练习 e204: 声明式导航链接 (link_nav)
//
// 目标: 使用 <A> 组件实现声明式导航链接，理解 <A> 与
//       原生 <a> 的区别。
//
// 难度: ⭐
// 核心知识点: <A> 组件
//
// TODO: 按照注释提示补全代码
// ============================================================

use leptos::prelude::*;
use leptos_router::components::Router;

// 定义三个页面组件
// 提示: 需要时添加 Routes, Route, A 等组件导入
// TODO: 补全 Home、Products、Contact 组件

#[component]
fn Home() -> impl IntoView {
    view! {
        <div>
            // TODO: 显示 "首页"
        </div>
    }
}

#[component]
fn Products() -> impl IntoView {
    view! {
        <div>
            // TODO: 显示 "产品中心"
        </div>
    }
}

#[component]
fn Contact() -> impl IntoView {
    view! {
        <div>
            // TODO: 显示 "联系我们"
        </div>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    // === 步骤 1 ——————————————————————————————————————————
    // TODO: 使用 <A> 组件创建导航菜单，包含三个链接：
    //       - 首页 (/)
    //       - 产品 (/products)
    //       - 联系 (/contact)
    // 提示: <A> 组件使用 href 属性指定目标路径

    view! {
        <Router>
            <nav>
                <ul>
                    // TODO: 使用 <li> 和 <A> 创建三个导航项
                </ul>
            </nav>
            <main>
                // TODO: 添加 <Routes fallback=|| "页面未找到"> 和三组 <Route>
                // 示例: <Route path="/" view=Home/>
            </main>
        </Router>
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
// use leptos_router::components::{Router, Routes, Route, A};
//
// #[component]
// fn Home() -> impl IntoView {
//     view! { <h2>"首页"</h2><p>"欢迎来到我们的网站"</p> }
// }
//
// #[component]
// fn Products() -> impl IntoView {
//     view! { <h2>"产品中心"</h2><p>"查看我们的产品"</p> }
// }
//
// #[component]
// fn Contact() -> impl IntoView {
//     view! { <h2>"联系我们"</h2><p>"通过邮件或电话联系我们"</p> }
// }
//
// #[component]
// fn Exercise() -> impl IntoView {
//     view! {
//         <Router>
//             <nav>
//                 <ul>
//                     <li><A href="/">"首页"</A></li>
//                     <li><A href="/products">"产品"</A></li>
//                     <li><A href="/contact">"联系"</A></li>
//                 </ul>
//             </nav>
//             <main>
//                 <Routes fallback=|| "页面未找到">
//                     <Route path="/" view=Home/>
//                     <Route path="/products" view=Products/>
//                     <Route path="/contact" view=Contact/>
//                 </Routes>
//             </main>
//         </Router>
//     }
// }
//
// fn main() {
//     mount_to_body(Exercise);
// }
// ```
//
// ### 知识点
// - <A> 是 Leptos 提供的声明式导航组件
// - <A> 会自动阻止默认的页面刷新行为，实现客户端路由
// - 相比原生 <a>，<A> 会与 Router 集成，无需手动管理导航
// - <A> 支持 style、class 等标准 HTML 属性
//
// </details>
