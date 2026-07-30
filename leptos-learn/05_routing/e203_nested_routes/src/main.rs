// ============================================================
// 练习 e203: 嵌套路由 (nested_routes)
//
// 目标: 使用嵌套 <Route> 实现布局共享，父路由提供
//       通用框架，子路由渲染具体内容。
//
// 难度: ⭐⭐
// 核心知识点: 嵌套 Route、<Outlet/>
//
// TODO: 按照注释提示补全代码
// ============================================================

use leptos::prelude::*;
use leptos_router::components::Router;

// === 步骤 1 ——————————————————————————————————————————
// TODO: 定义 ParentLayout 组件，包含 <Outlet/> 显示子路由内容
// 提示: 需要时添加 Routes, Route, Outlet, A 等组件导入
// 提示: <Outlet/> 会渲染匹配到的子路由组件

// === 步骤 2 ——————————————————————————————————————————
// TODO: 定义两个子组件 ChildA 和 ChildB

#[component]
fn ChildA() -> impl IntoView {
    view! {
        // TODO: 显示 "子页面 A"
    }
}

#[component]
fn ChildB() -> impl IntoView {
    view! {
        // TODO: 显示 "子页面 B"
    }
}

#[component]
fn Exercise() -> impl IntoView {
    // === 步骤 3 ——————————————————————————————————————————
    // TODO: 在 <Router> 中添加导航链接指向 /parent/a 和 /parent/b
    // TODO: 在 <Routes> 中定义路由：
    //       - path="/parent" 使用 ParentLayout，包含两个子 Route
    //         - path="/a" -> ChildA
    //         - path="/b" -> ChildB

    view! {
        <Router>
            <nav>
                // TODO: 导航链接
            </nav>
            <main>
                // TODO: <Routes> 和嵌套 <Route>
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
// use leptos_router::components::{Router, Routes, Route, Outlet, A};
//
// #[component]
// fn ParentLayout() -> impl IntoView {
//     view! {
//         <div style="border: 2px solid #4CAF50; padding: 1em; border-radius: 8px;">
//             <h2>"父布局"</h2>
//             <p>"这是共享的父布局框架"</p>
//             <hr/>
//             <Outlet/>
//         </div>
//     }
// }
//
// #[component]
// fn ChildA() -> impl IntoView {
//     view! { <p>"子页面 A 的内容"</p> }
// }
//
// #[component]
// fn ChildB() -> impl IntoView {
//     view! { <p>"子页面 B 的内容"</p> }
// }
//
// #[component]
// fn Exercise() -> impl IntoView {
//     view! {
//         <Router>
//             <nav>
//                 <A href="/parent/a">"子页面 A"</A>
//                 " | "
//                 <A href="/parent/b">"子页面 B"</A>
//             </nav>
//             <main>
//                 <Routes fallback=|| "页面未找到">
//                     <Route path="/parent" view=ParentLayout>
//                         <Route path="/a" view=ChildA/>
//                         <Route path="/b" view=ChildB/>
//                     </Route>
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
// - 嵌套路由通过在 <Route> 内部放置子 <Route> 实现
// - <Outlet/> 在父组件中渲染匹配到的子路由
// - 父路由路径会作为子路由路径的前缀
// - 子路由的 path 是相对于父路由的
//
// </details>
