// ============================================================
// 练习 e242: route_guard_resource — 使用 Resource 实现路由守卫
//
// 目标: 使用 LocalResource 异步验证认证状态，
//       未认证时显示 <Redirect/> 跳转首页
//
// 难度: ⭐⭐⭐
// 核心知识点: LocalResource、路由守卫、Outlet、Redirect
// ============================================================

use leptos::prelude::*;
use leptos_router::components::{Router, Routes, Route, ParentRoute, A, Redirect};
use leptos_router::components::Outlet;
use leptos_router::path;

// 模拟异步 token 验证
// TODO: 尝试将返回值改为 false 观察重定向效果
async fn validate_token() -> bool {
    true
}

// 首页 — 公开页面，无需登录
#[component]
fn Home() -> impl IntoView {
    view! {
        <div>
            <h2>"🏠 首页"</h2>
            <p>"这是一个公开页面，无需登录即可访问"</p>
            <p>"尝试点击"管理面板"链接，观察路由守卫行为"</p>
        </div>
    }
}

// 路由守卫组件 — 使用 LocalResource 异步鉴权
// TODO: 创建 LocalResource 调用 validate_token()
//       根据结果渲染 <Outlet/> 或 <Redirect path="/"/>
#[component]
fn AdminGuard() -> impl IntoView {
    // ★ 创建纯客户端资源来验证 token
    let auth = LocalResource::new(|| validate_token());

    view! {
        {move || {
            // .map() 返回 Option<T>，数据加载完成后为 Some
            auth.map(|valid| {
                if *valid {
                    view! { <Outlet/> }.into_any()
                } else {
                    view! { <Redirect path="/"/> }.into_any()
                }
            })
        }}
    }
}

// 受保护的页面 — 仅在验证通过后显示
#[component]
fn AdminPanel() -> impl IntoView {
    view! {
        <div style="border:2px solid #ff9800;padding:16px;border-radius:8px;">
            <h2>"🔒 管理面板"</h2>
            <p>"此内容仅在 token 验证通过后显示"</p>
            <p>"验证成功！欢迎进入管理区域。"</p>
        </div>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <Router>
            <nav>
                <A href="/">"首页 | "</A>
                <A href="/admin">"管理面板"</A>
            </nav>
            <hr/>
            <Routes fallback=|| view! { <p>"404 页面未找到"</p> }>
                <Route path=path!("/") view=Home/>
                <ParentRoute path=path!("/admin") view=AdminGuard>
                    <Route path=path!("/") view=AdminPanel/>
                </ParentRoute>
            </Routes>
        </Router>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(Exercise);
}

// ============================================================
// 参考答案 (思考后再看!)
// ============================================================
// <details>
// <summary>点击展开答案</summary>
//
// ### 路由守卫模式
// AdminGuard 通过 LocalResource 异步验证 token：
// - true → <Outlet/> 显示子路由
// - false → <Redirect/> 跳转首页
//
// ### LocalResource 要点
// - `LocalResource::new(|| async { })` 纯客户端资源，无需 serde
// - `.map(|v| ...)` 同步读取，返回 Option<T>
//
// </details>
