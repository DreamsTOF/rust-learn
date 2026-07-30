// ============================================================
// 练习 e201: 基础路由设置 (router_basic)
//
// 目标: 使用 <Router> <Routes> <Route> 设置基础路由，
//       实现首页和关于两个页面的切换。
//
// 难度: ⭐⭐
// 核心知识点: <Router>, <Routes>, <Route>
//
// TODO: 按照注释提示补全代码
// ============================================================

use leptos::prelude::*;
use leptos_router::components::Router;

// 提示: 定义两个页面组件 Home 和 About，分别渲染不同内容
//       需要时添加 Routes, Route, A 等组件导入
// TODO: 在下方定义 Home 组件

#[component]
fn Home() -> impl IntoView {
    view! {
        <div>
            // TODO: 渲染首页标题和欢迎文字
        </div>
    }
}

// TODO: 在下方定义 About 组件

#[component]
fn About() -> impl IntoView {
    view! {
        <div>
            // TODO: 渲染关于页面标题和内容
        </div>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    // === 步骤 1 ——————————————————————————————————————————
    // TODO: 在 <Router> 中添加导航链接
    //       - "/" 路径 -> "首页"
    //       - "/about" 路径 -> "关于"
    // TODO: 在 <Routes> 中定义两条路由规则
    //       - path="/" 渲染 Home 组件
    //       - path="/about" 渲染 About 组件
    //       设置 fallback 属性处理未匹配路径

    view! {
        <Router>
            <nav>
                // TODO: 添加导航链接（使用 <A> 组件）
            </nav>
            <main>
                // TODO: 添加 <Routes> 和 <Route>
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
//     view! {
//         <div>
//             <h2>"首页"</h2>
//             <p>"欢迎来到首页！"</p>
//         </div>
//     }
// }
//
// #[component]
// fn About() -> impl IntoView {
//     view! {
//         <div>
//             <h2>"关于"</h2>
//             <p>"这是关于页面。"</p>
//         </div>
//     }
// }
//
// #[component]
// fn Exercise() -> impl IntoView {
//     view! {
//         <Router>
//             <nav>
//                 <A href="/">"首页"</A>
//                 " | "
//                 <A href="/about">"关于"</A>
//             </nav>
//             <main>
//                 <Routes fallback=|| "页面未找到">
//                     <Route path="/" view=Home/>
//                     <Route path="/about" view=About/>
//                 </Routes>
//             </main>
//         </Router>
//     }
// }
//
// fn main() {
//     console_error_panic_hook::set_once();
//     mount_to_body(Exercise);
// }
// ```
//
// ### 知识点
// - <Router> 是路由容器，包裹整个路由系统
// - <Routes> 定义路由表，fallback 属性设置 404 页面
// - <Route> 的 path 匹配 URL 路径，view 指定要渲染的组件
// - <A> 是增强版链接组件，支持客户端路由导航
//
// </details>
