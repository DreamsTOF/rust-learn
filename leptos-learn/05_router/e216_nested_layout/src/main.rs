// ============================================================
// 练习 216: nested_layout — 嵌套路由布局
//
// 目标: 使用 ParentRoute 和 Outlet 实现共享导航布局
//
// 难度: ⭐⭐
// 核心知识点: ParentRoute, Outlet, 布局组件共享
//
// TODO: 按照注释提示补全代码
// ============================================================

use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::path;

// === 步骤 1 ——————————————————————————————————————————
// TODO: 创建 Layout 布局组件
// 提示: 包含 <nav> 导航链接和 <Outlet/> 子路由出口
#[component]
fn Layout() -> impl IntoView {
    view! {
        <nav>
            <a href="/">"首页"</a>
            <a href="/about">"关于"</a>
        </nav>
        // TODO: 在这里放置 Outlet 组件，用来渲染匹配的子路由
        <Outlet/>
    }
}

#[component]
fn Home() -> impl IntoView {
    view! {
        <h2>"首页"</h2>
        <p>"欢迎来到嵌套路由示例"</p>
    }
}

#[component]
fn About() -> impl IntoView {
    view! {
        <h2>"关于页面"</h2>
        <p>"这是嵌套路由布局中的子路由页面"</p>
    }
}

fn main() {
    mount_to_body(|| view! {
        // TODO: 配置路由器，使用 ParentRoute 包裹 Home 和 About 子路由
        // 提示: ParentRoute 用于定义共享布局包裹的子路由组
        <Router>
            <main>
                <Routes fallback=|| "页面未找到">
                    <ParentRoute path=path!("/") view=Layout>
                        <Route path=path!("/") view=Home/>
                        <Route path=path!("/about") view=About/>
                    </ParentRoute>
                </Routes>
            </main>
        </Router>
    });
}

// ============================================================
// 参考答案 (思考后再看!)
// ============================================================
// <details>
// <summary>点击展开答案</summary>
//
// ### 代码
// ```rust
// #[component]
// fn Layout() -> impl IntoView {
//     view! {
//         <nav>
//             <a href="/">"首页"</a>
//             <a href="/about">"关于"</a>
//         </nav>
//         <Outlet/>
//     }
// }
//
// fn main() {
//     mount_to_body(|| view! {
//         <Router>
//             <main>
//                 <Routes fallback=|| "页面未找到">
//                     <ParentRoute path=path!("/") view=Layout>
//                         <Route path=path!("/") view=Home/>
//                         <Route path=path!("/about") view=About/>
//                     </ParentRoute>
//                 </Routes>
//             </main>
//         </Router>
//     });
// }
// ```
//
// ### 知识点
// - `<ParentRoute>` 定义一个共享布局区域，包裹一组子路由
// - `<Outlet/>` 是子路由内容的渲染出口，放在布局组件中
// - 布局组件中的导航在所有子路由页面间共享
// - 访问 /about 时，Layout 依然保持，只有 Outlet 内的内容切换
//
// </details>
