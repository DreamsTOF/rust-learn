// ============================================================
// 练习 e210: 重定向 (redirect)
//
// 目标: 使用 <Redirect/> 组件实现条件重定向，
//       结合 use_navigate() 完成登录状态切换时的导航。
//
// 难度: ⭐⭐
// 核心知识点: <Redirect/> 组件, 条件重定向, use_navigate()
//
// 场景:
//   - /dashboard 页面需要登录才能访问
//   - 未登录用户访问 /dashboard 会被重定向到 /login
//   - /login 页面提供登录按钮，登录后跳转到 /dashboard
//
// TODO: 按照注释提示补全代码
// ============================================================

use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::hooks::*;

// ============================================================
// 步骤 1 — 创建登录页面组件
// ============================================================

#[component]
fn Login() -> impl IntoView {
    // TODO: 从 Context 中获取 logged_in 信号
    // let logged_in = use_context::<RwSignal<bool>>().expect("logged_in not found");
    //
    // TODO: 调用 use_navigate() 获取 navigate 函数
    // let navigate = use_navigate();
    //
    // 在按钮点击事件中:
    //   logged_in.set(true);
    //   navigate("/dashboard", Default::default());

    view! {
        <h2>"登录"</h2>
        <p>"这是登录页面。请先登录。"</p>
        // TODO: 添加一个按钮 "点击登录"，实现登录和跳转
    }
}

// ============================================================
// 步骤 2 — 创建仪表盘页面组件（需要登录）
// ============================================================

#[component]
fn Dashboard() -> impl IntoView {
    // TODO: 从 Context 中获取 logged_in 信号
    // let logged_in = use_context::<RwSignal<bool>>().expect("logged_in not found");
    //
    // 使用条件渲染: 如果 !logged_in() 则渲染 <Redirect path="/login"/>
    // 否则显示仪表盘内容
    // 提示: 使用 .into_any() 统一分支类型

    view! {
        // TODO: 使用 move || { ... } 闭包做条件判断
        // 未登录 -> <Redirect path="/login"/>
        // 已登录 -> 显示仪表盘内容
    }
}

// ============================================================
// 步骤 3 — 组装根组件
// ============================================================

#[component]
fn Exercise() -> impl IntoView {
    // TODO: 创建 RwSignal<bool>，初始值为 false（未登录）
    // let logged_in = RwSignal::new(false);
    //
    // TODO: 使用 provide_context(logged_in) 提供给子组件
    //
    // TODO: 在 <Router> 中:
    //   - <nav> 添加 <A href="/dashboard">"仪表盘"</A>
    //   - <Routes> 中添加两条路由:
    //     - path="/login" view=Login
    //     - path="/dashboard" view=Dashboard

    view! {
        // TODO: 组装完整的路由结构
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
use leptos_router::hooks::*;
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
//                     <Route path="/login" view=Login/>
//                     <Route path="/dashboard" view=Dashboard/>
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
// - <Redirect/> 组件在渲染时执行重定向，适用于条件路由守卫
// - use_navigate() 可以在事件处理中执行编程式重定向
// - provide_context/use_context 可以在组件树中共享状态
// - .into_any() 用于统一条件渲染中不同分支的类型
//
// </details>
