// ============================================================
// 练习 e206: 基础导航 (a_nav)
//
// 目标: 使用 <Router> <Routes> <Route> 设置路由，
//       并用 <A> 组件实现声明式导航。
//
// 难度: ⭐
// 核心知识点: <Router>, <Routes>, <Route>, <A>
//
// TODO: 按照注释提示补全代码
// ============================================================

use leptos::prelude::*;
use leptos_router::components::*;

// ============================================================
// 步骤 1 — 定义三个页面组件
// ============================================================

// TODO: 创建 Home 组件，显示首页标题和欢迎文字
#[component]
fn Home() -> impl IntoView {
    view! {
        <div>
            // TODO: 填入 <h2>"首页"</h2> 和 <p>"欢迎来到首页！"</p>
        </div>
    }
}

// TODO: 创建 About 组件，显示关于页面标题和内容
#[component]
fn About() -> impl IntoView {
    view! {
        <div>
            // TODO: 填入 <h2>"关于"</h2> 和 <p>"这是关于页面。"</p>
        </div>
    }
}

// TODO: 创建 Contact 组件，显示联系方式页面标题和内容
#[component]
fn Contact() -> impl IntoView {
    view! {
        <div>
            // TODO: 填入 <h2>"联系方式"</h2> 和 <p>"请联系我们。"</p>
        </div>
    }
}

// ============================================================
// 步骤 2 — 组装根组件 Exercise
// ============================================================

#[component]
fn Exercise() -> impl IntoView {
    // === 子步骤 2a: 在 <Router> 中使用 <A> 创建导航链接 ===
    // TODO: 在 <nav> 中添加三个 <A> 链接，分别指向
    //       - "/home" -> "首页"
    //       - "/about" -> "关于"
    //       - "/contact" -> "联系方式"
    //
    // === 子步骤 2b: 在 <main> 中添加 <Routes fallback=...> ===
    // TODO: 在 <Routes fallback=|| "页面未找到"> 中定义三个 <Route>
    //       - path="/home" view=Home
    //       - path="/about" view=About
    //       - path="/contact" view=Contact

    view! {
        <Router>
            <nav>
                // TODO: 添加导航链接
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
// use leptos_router::components::*;
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
//                     <Route path="/home" view=Home/>
//                     <Route path="/about" view=About/>
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
// - <Router> 是路由容器，包裹整个路由系统
// - <Routes> 定义路由表，fallback 属性指定未匹配路径的显示内容
// - <Route> 的 path 匹配 URL 路径，view 指定渲染的组件
// - <A> 是增强版链接组件，支持客户端路由导航，不会触发页面刷新
//
// </details>
