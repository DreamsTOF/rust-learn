// ============================================================
// Exercise 205 - Answer: Active Link Highlighting
// ============================================================

use leptos::prelude::*;
use leptos_router::path;
use leptos_router::components::{Router, Routes, Route, A};

#[component]
fn Dashboard() -> impl IntoView {
    view! { <h2>"仪表盘"</h2><p>"欢迎回来！这是你的控制面板。"</p> }
}

#[component]
fn Settings() -> impl IntoView {
    view! { <h2>"设置"</h2><p>"在这里管理你的偏好设置。"</p> }
}

#[component]
fn Profile() -> impl IntoView {
    view! { <h2>"个人资料"</h2><p>"查看和编辑你的个人信息。"</p> }
}

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <Router>
            <nav>
                <A href="/dashboard">"仪表盘"</A>
                <A href="/settings">"设置"</A>
                <A href="/profile">"个人资料"</A>
            </nav>
            <main>
                <Routes fallback=|| "页面未找到">
                    <Route path=path!("/dashboard") view=Dashboard/>
                    <Route path=path!("/settings") view=Settings/>
                    <Route path=path!("/profile") view=Profile/>
                </Routes>
            </main>
        </Router>
    }
}

fn main() {
    mount_to_body(Exercise);
}
