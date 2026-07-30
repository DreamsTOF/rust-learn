// ============================================================
// Exercise 241 - Answer: route_outlet_context
// ============================================================

use leptos::prelude::*;
use leptos_router::components::{Router, Routes, Route, ParentRoute, A};
use leptos_router::components::Outlet;
use leptos_router::path;

#[derive(Clone, Debug)]
struct User {
    name: String,
    role: String,
}

#[component]
fn Home() -> impl IntoView {
    view! {
        <div>
            <h2>"🏠 首页"</h2>
            <p>"欢迎来到路由 context 示例"</p>
        </div>
    }
}

#[component]
fn DashboardLayout() -> impl IntoView {
    let user = User {
        name: "Alice".into(),
        role: "管理员".into(),
    };
    provide_context(user);

    view! {
        <div style="border:2px solid #4CAF50;padding:16px;border-radius:8px;">
            <h2>"📊 控制面板"</h2>
            <p style="color:#666;">"下方由 Outlet 渲染子路由内容:"</p>
            <hr/>
            <Outlet/>
        </div>
    }
}

#[component]
fn Profile() -> impl IntoView {
    let user = use_context::<User>()
        .expect("User context 未提供");

    view! {
        <div style="background:#f0f8ff;padding:12px;border-radius:4px;margin-top:8px;">
            <h3>"👤 个人资料"</h3>
            <p>"姓名: " {user.name.clone()}</p>
            <p>"角色: " {user.role.clone()}</p>
        </div>
    }
}

#[component]
fn Settings() -> impl IntoView {
    let user = use_context::<User>()
        .expect("User context 未提供");

    view! {
        <div style="background:#fff0f5;padding:12px;border-radius:4px;margin-top:8px;">
            <h3>"⚙️ 设置"</h3>
            <p>"当前用户: " {user.name.clone()}</p>
            <p>"角色权限: " {user.role.clone()}</p>
        </div>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <Router>
            <nav>
                <A href="/">"首页 | "</A>
                <A href="/dashboard">"控制面板 | "</A>
                <A href="/dashboard/profile">"个人资料 | "</A>
                <A href="/dashboard/settings">"设置"</A>
            </nav>
            <hr/>
            <Routes fallback=|| view! { <p>"404 页面未找到"</p> }>
                <Route path=path!("/") view=Home/>
                <ParentRoute path=path!("/dashboard") view=DashboardLayout>
                    <Route path=path!("/") view=move || view! { <p>"请从上方导航选择子页面"</p> }/>
                    <Route path=path!("/profile") view=Profile/>
                    <Route path=path!("/settings") view=Settings/>
                </ParentRoute>
            </Routes>
        </Router>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(Exercise);
}
