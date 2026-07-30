// ============================================================
// 练习 e206: 编程式导航 (navigate_programmatic)
//
// 目标: 使用 use_navigate() 钩子实现编程式导航，
//       在按钮点击时通过代码跳转到不同页面。
//
// 难度: ⭐⭐
// 核心知识点: use_navigate, NavigateOptions
//
// TODO: 按照注释提示补全代码
// ============================================================

use leptos::prelude::*;
use leptos_router::components::Router;

// 提示: 需要时添加 Routes, Route, A 等组件导入
// 提示: use_navigate 来自 leptos_router::hooks
// TODO: 定义 Home 和 About 两个页面组件

#[component]
fn Home() -> impl IntoView {
    view! {
        <div>
            // TODO: 显示首页标题和内容
        </div>
    }
}

#[component]
fn About() -> impl IntoView {
    view! {
        <div>
            // TODO: 显示关于页面标题和内容
        </div>
    }
}

#[component]
fn NavButtons() -> impl IntoView {
    // === 步骤 1 ——————————————————————————————————————————
    // TODO: 使用 use_navigate() 获取导航函数
    // TODO: 创建两个按钮，点击时分别导航到 /home 和 /about
    // 提示: navigate("/path", Default::default())
    // 提示: 如果两个按钮都需要 navigate，记得先 .clone()

    view! {
        <nav>
            // TODO: "Go Home" 按钮
            // TODO: "Go About" 按钮
        </nav>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    // === 步骤 2 ——————————————————————————————————————————
    // TODO: 在 <Router> 中放置 <NavButtons/> 和 <Routes>
    // TODO: 在 <Routes> 中定义两条路由：
    //       - path="/home" -> Home
    //       - path="/about" -> About
    //       设置 fallback 属性

    view! {
        <Router>
            <NavButtons/>
            <main>
                // TODO: 添加 <Routes fallback=|| "页面未找到">
                //       <Route path=path!("/home") view=Home/>
                //       <Route path=path!("/about") view=About/>
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
// fn Home() -> impl IntoView {
//     view! {
//         <h2>"首页"</h2>
//         <p>"欢迎来到首页！"</p>
//     }
// }
//
// #[component]
// fn About() -> impl IntoView {
//     view! {
//         <h2>"关于"</h2>
//         <p>"这是关于页面。"</p>
//     }
// }
//
// #[component]
// fn NavButtons() -> impl IntoView {
//     let navigate = use_navigate();
//     let navigate_to_about = navigate.clone();
//
//     view! {
//         <nav>
//             <button on:click=move |_| navigate("/home", Default::default())>
//                 "Go Home"
//             </button>
//             " "
//             <button on:click=move |_| navigate_to_about("/about", Default::default())>
//                 "Go About"
//             </button>
//         </nav>
//     }
// }
//
// #[component]
// fn Exercise() -> impl IntoView {
//     view! {
//         <Router>
//             <NavButtons/>
//             <main>
//                 <Routes fallback=|| "页面未找到">
//                     <Route path=path!("/home") view=Home/>
//                     <Route path=path!("/about") view=About/>
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
// - use_navigate() 返回一个导航函数，可在事件处理中调用
// - NavigateOptions 控制导航行为，Default::default() 使用默认选项
// - 编程式导航在按钮点击、表单提交等场景非常有用
// - 多个 move 闭包使用 navigate 时需要先 clone
//
// </details>
