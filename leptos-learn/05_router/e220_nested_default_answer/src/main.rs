// ============================================================
// Exercise 220 - Answer
// nested_default — 嵌套路由的默认子路由
// ============================================================

use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::path;

#[component]
fn DashboardLayout() -> impl IntoView {
    view! {
        <nav>
            <a href="/dashboard">"概览"</a>
            <a href="/dashboard/analytics">"分析"</a>
            <a href="/dashboard/reports">"报表"</a>
        </nav>
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

#[component]
fn Exercise() -> impl IntoView {
    view! {
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
    }
}

fn main() {
    mount_to_body(Exercise);
}
