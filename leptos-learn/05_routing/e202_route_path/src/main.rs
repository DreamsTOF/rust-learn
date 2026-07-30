// ============================================================
// 练习 e202: 路径匹配规则 (route_path)
//
// 目标: 理解 <Route> 的 path 属性如何匹配不同的静态路径。
//
// 难度: ⭐⭐
// 核心知识点: path 属性、静态路径匹配
//
// TODO: 按照注释提示补全代码
// ============================================================

use leptos::prelude::*;
use leptos_router::components::Router;

// 定义三个页面组件 PageA、PageB、PageC
// 提示: 需要时添加 Routes, Route, A 等组件导入
// TODO: 补全 PageA 组件，显示 "页面 A"
// TODO: 补全 PageB 组件，显示 "页面 B"
// TODO: 补全 PageC 组件，显示 "页面 C"

#[component]
fn PageA() -> impl IntoView {
    view! {
        <div>
            // TODO: 显示 "你正在访问页面 A"
        </div>
    }
}

#[component]
fn PageB() -> impl IntoView {
    view! {
        <div>
            // TODO: 显示 "你正在访问页面 B"
        </div>
    }
}

#[component]
fn PageC() -> impl IntoView {
    view! {
        <div>
            // TODO: 显示 "你正在访问页面 C"
        </div>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    // === 步骤 1 ——————————————————————————————————————————
    // TODO: 创建三个导航链接指向 /a, /b, /c
    // TODO: 在 <Routes> 中定义三条路由规则
    //       path="/a" -> PageA
    //       path="/b" -> PageB
    //       path="/c" -> PageC

    view! {
        <Router>
            <nav>
                // TODO: 添加导航链接
            </nav>
            <main>
                // TODO: 添加 <Routes> 和三条 <Route>
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
// fn PageA() -> impl IntoView {
//     view! { <h3>"你正在访问页面 A"</h3> }
// }
//
// #[component]
// fn PageB() -> impl IntoView {
//     view! { <h3>"你正在访问页面 B"</h3> }
// }
//
// #[component]
// fn PageC() -> impl IntoView {
//     view! { <h3>"你正在访问页面 C"</h3> }
// }
//
// #[component]
// fn Exercise() -> impl IntoView {
//     view! {
//         <Router>
//             <nav>
//                 <A href="/a">"Page A"</A>
//                 " | "
//                 <A href="/b">"Page B"</A>
//                 " | "
//                 <A href="/c">"Page C"</A>
//             </nav>
//             <main>
//                 <Routes fallback=|| "页面未找到">
//                     <Route path="/a" view=PageA/>
//                     <Route path="/b" view=PageB/>
//                     <Route path="/c" view=PageC/>
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
// - path 属性使用字符串定义 URL 路径
// - 静态路径是精确匹配，如 "/a" 只匹配 /a
// - <Routes> 会按定义顺序匹配第一个成功的路由
// - fallback 属性在没有路由匹配时显示
//
// </details>
