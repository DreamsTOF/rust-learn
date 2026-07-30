// ============================================================
// Exercise 242 - Answer: route_guard_resource
// ============================================================

use leptos::prelude::*;
use leptos_router::components::{Router, Routes, Route, ParentRoute, A, Redirect};
use leptos_router::components::Outlet;
use leptos_router::path;

async fn validate_token() -> bool {
    true
}

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

#[component]
fn AdminGuard() -> impl IntoView {
    let auth = LocalResource::new(|| validate_token());

    view! {
        {move || {
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
