// ============================================================
// 练习 e210: 路由基础 (hash_routing)
//
// 目标: 使用 <Router> <Routes> <Route> 设置完整路由系统，
//       实现首页、关于、联系方式三个页面的导航切换。
//
// 难度: ⭐⭐
// 核心知识点: <Router>, <Routes>, <Route>, <A>
//
// TODO: 按照注释提示补全代码
// ============================================================

use leptos::prelude::*;
use leptos_router::components::Router;

// 提示: 需要时添加 Routes, Route, A 等组件导入
// TODO: 定义 Home 组件

#[component]
fn Home() -> impl IntoView {
    view! {
        <div>
            // TODO: 显示首页标题和欢迎文字
        </div>
    }
}

// TODO: 定义 About 组件

#[component]
fn About() -> impl IntoView {
    view! {
        <div>
            // TODO: 显示关于页面标题和内容
        </div>
    }
}

// TODO: 定义 Contact 组件

#[component]
fn Contact() -> impl IntoView {
    view! {
        <div>
            // TODO: 显示联系方式页面标题和内容
        </div>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    // === 步骤 1 ——————————————————————————————————————————
    // TODO: 在 <Router> 中创建导航菜单（使用 <A> 组件）
    //       - "/home" -> "首页"
    //       - "/about" -> "关于"
    //       - "/contact" -> "联系方式"
    // TODO: 在 <Routes> 中定义三条路由规则
    //       设置 fallback 属性处理未匹配路径

    view! {
        <Router>
            <nav>
                // TODO: 添加三个 <A> 导航链接
            </nav>
            <main>
                // TODO: 添加 <Routes fallback=|| "页面未找到">
                //       <Route path=path!("/home") view=Home/>
                //       <Route path=path!("/about") view=About/>
                //       <Route path=path!("/contact") view=Contact/>
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
// use leptos_router::components::*;
// use leptos_router::path;
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
// fn Contact() -> impl IntoView {
//     view! {
//         <div>
//             <h2>"联系方式"</h2>
//             <p>"请联系我们。"</p>
//         </div>
//     }
// }
//
// #[component]
// fn Exercise() -> impl IntoView {
//     view! {
//         <Router>
//             <nav>
//                 <A href="/home">"首页"</A>
//                 " | "
//                 <A href="/about">"关于"</A>
//                 " | "
//                 <A href="/contact">"联系方式"</A>
//             </nav>
//             <main>
//                 <Routes fallback=|| "页面未找到">
//                     <Route path=path!("/home") view=Home/>
//                     <Route path=path!("/about") view=About/>
//                     <Route path=path!("/contact") view=Contact/>
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
// - <Router> 是路由容器，包裹整个应用的路由系统
// - <Routes> 定义路由表，fallback 处理未匹配路径
// - <Route> 的 path 匹配 URL，view 指定渲染组件
// - <A> 是增强版链接组件，支持客户端路由导航
//
// </details>
