// ============================================================
// Exercise 245 - Answer: multi_layout
// ============================================================

use leptos::prelude::*;
use leptos_router::components::{Router, Routes, Route, ParentRoute, A};
use leptos_router::components::Outlet;
use leptos_router::path;

#[component]
fn SidebarLayout() -> impl IntoView {
    view! {
        <div style="display:flex;gap:16px;min-height:300px;">
            <div style="width:200px;background:#f5f5f5;padding:16px;border-radius:8px;">
                <h3>"📂 侧边栏"</h3>
                <ul style="list-style:none;padding:0;">
                    <li><A href="/dashboard">"控制面板"</A></li>
                    <li><A href="/dashboard/analytics">"数据分析"</A></li>
                    <li><A href="/dashboard/reports">"报告"</A></li>
                </ul>
            </div>
            <div style="flex:1;padding:16px;border:1px solid #ddd;border-radius:8px;">
                <Outlet/>
            </div>
        </div>
    }
}

#[component]
fn TopbarLayout() -> impl IntoView {
    view! {
        <div>
            <div style="background:#ff9800;padding:12px 16px;border-radius:8px 8px 0 0;color:white;">
                <h3 style="margin:0;">"🔝 顶部导航布局"</h3>
                <div style="margin-top:8px;">
                    <A href="/about">"关于我们 | "</A>
                    <A href="/contact">"联系我们 | "</A>
                    <A href="/help">"帮助"</A>
                </div>
            </div>
            <div style="padding:16px;border:1px solid #ddd;border-top:none;border-radius:0 0 8px 8px;">
                <Outlet/>
            </div>
        </div>
    }
}

#[component]
fn Home() -> impl IntoView {
    view! {
        <div>
            <h2>"🏠 首页"</h2>
            <p>"不同路由使用不同布局的演示"</p>
        </div>
    }
}

#[component]
fn DashboardHome() -> impl IntoView {
    view! { <p>"欢迎来到控制面板"</p> }
}

#[component]
fn Analytics() -> impl IntoView {
    view! { <p>"📊 数据分析页面"</p> }
}

#[component]
fn Reports() -> impl IntoView {
    view! { <p>"📄 报告页面"</p> }
}

#[component]
fn About() -> impl IntoView {
    view! {
        <div>
            <h2>"📖 关于我们"</h2>
            <p>"这是一个使用顶部导航布局的页面"</p>
        </div>
    }
}

#[component]
fn Contact() -> impl IntoView {
    view! {
        <div>
            <h2>"📧 联系我们"</h2>
            <p>"发送邮件到 example@test.com"</p>
        </div>
    }
}

#[component]
fn Help() -> impl IntoView {
    view! {
        <div>
            <h2>"❓ 帮助中心"</h2>
            <p>"常见问题解答"</p>
        </div>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <Router>
            <nav>
                <A href="/">"🏠 首页 | "</A>
                <A href="/dashboard">"📊 仪表盘 | "</A>
                <A href="/about">"📖 关于 | "</A>
                <A href="/contact">"📧 联系"</A>
            </nav>
            <hr/>
            <Routes fallback=|| view! { <p>"404 页面未找到"</p> }>
                <Route path=path!("/") view=Home/>
                <ParentRoute path=path!("/dashboard") view=SidebarLayout>
                    <Route path=path!("/") view=DashboardHome/>
                    <Route path=path!("/analytics") view=Analytics/>
                    <Route path=path!("/reports") view=Reports/>
                </ParentRoute>
                <ParentRoute path=path!("/about") view=TopbarLayout>
                    <Route path=path!("/") view=About/>
                </ParentRoute>
                <ParentRoute path=path!("/contact") view=TopbarLayout>
                    <Route path=path!("/") view=Contact/>
                </ParentRoute>
                <ParentRoute path=path!("/help") view=TopbarLayout>
                    <Route path=path!("/") view=Help/>
                </ParentRoute>
            </Routes>
        </Router>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(Exercise);
}
