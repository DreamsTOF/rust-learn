// ============================================================
// 练习 220: nested_default — 嵌套路由的默认子路由
//
// 目标: 在嵌套路由中设置默认子路由（path=""）
//       访问父路由时自动显示默认子路由内容
//
// 难度: ⭐⭐
// 核心知识点: 默认子路由, index 路由
//
// TODO: 按照注释提示补全代码
// ============================================================

use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::path;

// === 步骤 1 ——————————————————————————————————————————
// TODO: 创建 DashboardLayout 布局组件
// 提示: 包含导航链接和 <Outlet/>
#[component]
fn DashboardLayout() -> impl IntoView {
    view! {
        <nav>
            <a href="/dashboard">"概览"</a>
            <a href="/dashboard/analytics">"分析"</a>
            <a href="/dashboard/reports">"报表"</a>
        </nav>
        // TODO: 在这里放置 Outlet 组件
        <Outlet/>
    }
}

#[component]
fn Overview() -> impl IntoView {
    view! {
        <h3>"概览"</h3>
        <p>"欢迎回到控制台。这是默认显示的页面。"</p>
        <p>"今日访问量: 1,234"</p>
    }
}

#[component]
fn Analytics() -> impl IntoView {
    view! {
        <h3>"数据分析"</h3>
        <p>"图表和统计信息将在此处显示。"</p>
    }
}

#[component]
fn Reports() -> impl IntoView {
    view! {
        <h3>"报表"</h3>
        <p>"查看和下载月度报表。"</p>
    }
}

fn main() {
    mount_to_body(|| view! {
        // TODO: 配置路由器，确保 /dashboard 默认显示 Overview
        // 提示: path="/dashboard" 的子路由作为默认子路由
        <Router>
            <main>
                <h2>"控制台"</h2>
                <Routes fallback=|| "页面未找到">
                    <ParentRoute path=path!("/dashboard") view=DashboardLayout>
                        <Route path=path!("/dashboard") view=Overview/>
                        <Route path=path!("/dashboard/analytics") view=Analytics/>
                        <Route path=path!("/dashboard/reports") view=Reports/>
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
// fn main() {
//     mount_to_body(|| view! {
//         <Router>
//             <main>
//                 <h2>"控制台"</h2>
//                 <Routes fallback=|| "页面未找到">
//                     <ParentRoute path=path!("/dashboard") view=DashboardLayout>
//                         <Route path=path!("/dashboard") view=Overview/>
//                         <Route path=path!("/dashboard/analytics") view=Analytics/>
//                         <Route path=path!("/dashboard/reports") view=Reports/>
//                     </ParentRoute>
//                 </Routes>
//             </main>
//         </Router>
//     });
// }
// ```
//
// ### 知识点
// - 在 ParentRoute 中，第一个子 Route 的 path 与父路由 path 相同时，它作为默认子路由
// - 访问 /dashboard 时，DashboardLayout 显示，且 Outlet 自动渲染 Overview
// - 这种 pattern 类似传统 SPA 框架中的 index 路由
// - 默认子路由确保用户访问父级 URL 时不会看到空白内容
//
// </details>
