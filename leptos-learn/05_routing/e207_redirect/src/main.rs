// ============================================================
// 练习 e207: 重定向 (redirect)
//
// 目标: 使用 <Redirect/> 组件和 use_navigate() 实现
//       条件重定向，未登录用户访问仪表盘时自动跳转
//       到登录页。
//
// 难度: ⭐⭐⭐
// 核心知识点: <Redirect/> 组件, 路由守卫
//
// TODO: 按照注释提示补全代码
// ============================================================

use leptos::prelude::*;
use leptos_router::components::Router;

// 提示: 需要时添加 Routes, Route, A, Redirect 等组件导入
// 提示: use_navigate 来自 leptos_router::hooks
// 提示: RwSignal, provide_context, use_context 来自 leptos::prelude

// TODO: 定义 Login 组件
//       1. 通过 use_context::<RwSignal<bool>>() 获取登录状态
//       2. 使用 use_navigate() 获取导航函数
//       3. 点击按钮时设置 logged_in.set(true) 并导航到 /dashboard

#[component]
fn Login() -> impl IntoView {
    view! {
        <h2>"登录"</h2>
        <p>"这是登录页面。请先登录。"</p>
        // TODO: 添加按钮，点击时设置登录状态并跳转到仪表盘
    }
}

// TODO: 定义 Dashboard 组件
//       1. 通过 use_context::<RwSignal<bool>>() 获取登录状态
//       2. 如果未登录 (logged_in() 为 false)，渲染 <Redirect path="/login"/>
//       3. 已登录则显示仪表盘内容

#[component]
fn Dashboard() -> impl IntoView {
    view! {
        // TODO: 条件渲染：未登录时重定向，已登录显示欢迎信息
    }
}

#[component]
fn Exercise() -> impl IntoView {
    // === 步骤 1 ——————————————————————————————————————————
    // TODO: 创建 RwSignal::new(false) 作为登录状态
    // TODO: 使用 provide_context() 共享登录状态
    // TODO: 在 <Router> 中创建导航链接和路由表
    //       路由：
    //       - path="/login" -> Login
    //       - path="/dashboard" -> Dashboard

    view! {
        // TODO: 完成 Router 和路由配置
        <Router>
            <nav>
                // TODO: 添加导航链接到 /dashboard
            </nav>
            <main>
                // TODO: 添加 <Routes> 和两条 <Route>
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
// use leptos_router::hooks::*;
// use leptos_router::path;
//
// #[component]
// fn Login() -> impl IntoView {
//     let logged_in = use_context::<RwSignal<bool>>().expect("logged_in not found");
//     let navigate = use_navigate();
//
//     view! {
//         <h2>"登录"</h2>
//         <p>"这是登录页面。请先登录。"</p>
//         <button on:click=move |_| {
//             logged_in.set(true);
//             navigate("/dashboard", Default::default());
//         }>"点击登录"</button>
//     }
// }
//
// #[component]
// fn Dashboard() -> impl IntoView {
//     let logged_in = use_context::<RwSignal<bool>>().expect("logged_in not found");
//
//     view! {
//         {move || if !logged_in() {
//             view! { <Redirect path="/login"/> }.into_any()
//         } else {
//             view! {
//                 <h2>"仪表盘"</h2>
//                 <p>"欢迎回来！"</p>
//             }.into_any()
//         }}
//     }
// }
//
// #[component]
// fn Exercise() -> impl IntoView {
//     let logged_in = RwSignal::new(false);
//     provide_context(logged_in);
//
//     view! {
//         <Router>
//             <nav>
//                 <A href="/dashboard">"仪表盘"</A>
//             </nav>
//             <main>
//                 <Routes fallback=|| "页面未找到">
//                     <Route path=path!("/login") view=Login/>
//                     <Route path=path!("/dashboard") view=Dashboard/>
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
// - <Redirect/> 组件在渲染时自动导航到指定路径
// - use_navigate() 可在事件处理中手动触发导航
// - provide_context / use_context 用于组件间共享数据
// - 条件渲染 + Redirect 可实现路由守卫效果
//
// </details>
