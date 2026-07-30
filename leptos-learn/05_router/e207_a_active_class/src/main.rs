// ============================================================
// 练习 e207: 导航激活样式 (a_active_class)
//
// 目标: 使用 <A> 的 class:active 属性，
//       为当前激活的路由链接自动添加样式。
//
// 难度: ⭐⭐
// 核心知识点: <A class:active>, 条件 CSS 类
//
// TODO: 按照注释提示补全代码
// ============================================================

use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::path;

#[component]
fn Home() -> impl IntoView {
    view! {
        <div>
            <h2>"首页"</h2>
            <p>"欢迎来到首页！"</p>
        </div>
    }
}

#[component]
fn About() -> impl IntoView {
    view! {
        <div>
            <h2>"关于"</h2>
            <p>"这是关于页面。"</p>
        </div>
    }
}

#[component]
fn Contact() -> impl IntoView {
    view! {
        <div>
            <h2>"联系方式"</h2>
            <p>"请联系我们。"</p>
        </div>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    // === 步骤 1 — 在 <A> 上添加 class:active 属性 ===
    // TODO: 在每个 <A> 上添加 class:active 属性。
    //       当链接对应的路由处于激活状态时，<A> 会自动添加指定 CSS 类。
    //       如: <A href="/home" class:active>...</A>
    //
    //       也可以指定自定义类名:
    //       <A href="/home" class:active="my-active">...</A>
    //
    // 提示: 使用 class:active (不带值) 会添加 CSS 类 "active"
    //
    // === 步骤 2 — 添加 CSS 样式 ===
    // TODO: 使用 <style> 标签定义 .active 样式
    //       例如: 粗体 + 颜色，让当前导航项视觉上突出

    view! {
        <Router>
            // TODO: 添加 <style> 标签定义 .active { font-weight: bold; color: red; }
            //       参考: <style>"\n    .active {\n        font-weight: bold;\n        color: #c00;\n    }\n"</style>
            <nav>
                <A href="/home">"首页"</A>
                " | "
                <A href="/about">"关于"</A>
                " | "
                <A href="/contact">"联系方式"</A>
            </nav>
            <main>
                <Routes fallback=|| "页面未找到">
                    <Route path=path!("/home") view=Home/>
                    <Route path=path!("/about") view=About/>
                    <Route path=path!("/contact") view=Contact/>
                </Routes>
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
//             <style>
//                 "\n    .active {\n        font-weight: bold;\n        color: #c00;\n    }\n"
//             </style>
//             <nav>
//                 <A href="/home" class:active>"首页"</A>
//                 " | "
//                 <A href="/about" class:active>"关于"</A>
//                 " | "
//                 <A href="/contact" class:active>"联系方式"</A>
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
// - <A> 的 `class:active` 属性：当链接的 href 匹配当前路由时，
//   自动添加 CSS 类 `active`（或自定义类名）
// - 配合 CSS 样式可以让当前导航项在视觉上突出
//
// </details>
