// ============================================================
// Exercise 243 - Answer: route_preload
// ============================================================

use leptos::prelude::*;
use leptos_router::components::{Router, Routes, Route, ParentRoute, A};
use leptos_router::components::Outlet;
use leptos_router::path;

#[derive(Clone, Debug)]
struct DashboardData {
    title: String,
    content: String,
}

async fn load_dashboard_data() -> DashboardData {
    DashboardData {
        title: "仪表盘数据".into(),
        content: "这是一条在路由渲染前预加载的数据。\n\nLocalResource 会在组件第一次创建时自动触发异步加载，子组件通过 .map() 获取数据。".into(),
    }
}

#[component]
fn DataLoader() -> impl IntoView {
    let data = LocalResource::new(|| load_dashboard_data());
    provide_context(data);

    view! {
        <div style="border:2px solid #2196F3;padding:16px;border-radius:8px;">
            <h2>"📦 数据加载层"</h2>
            <Suspense fallback=|| view! { <p>"⏳ 数据加载中..."</p> }>
                <Outlet/>
            </Suspense>
        </div>
    }
}

#[component]
fn Dashboard() -> impl IntoView {
    let data = use_context::<LocalResource<DashboardData>>()
        .expect("DataLoader 未提供 data Resource");

    view! {
        {move || {
            data.map(|data| {
                view! {
                    <div style="background:#e3f2fd;padding:16px;border-radius:4px;margin-top:8px;">
                        <h3>{data.title.clone()}</h3>
                        <pre style="white-space:pre-wrap;font-family:inherit;">{data.content.clone()}</pre>
                    </div>
                }
                .into_any()
            })
        }}
    }
}

#[component]
fn Home() -> impl IntoView {
    view! {
        <div>
            <h2>"🏠 首页"</h2>
            <p>"点击"仪表盘"链接观察预加载效果"</p>
        </div>
    }
}

#[component]
fn About() -> impl IntoView {
    view! {
        <div>
            <h2>"ℹ️ 关于"</h2>
            <p>"此页面没有数据预加载"</p>
        </div>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <Router>
            <nav>
                <A href="/">"首页 | "</A>
                <A href="/dashboard">"仪表盘 | "</A>
                <A href="/about">"关于"</A>
            </nav>
            <hr/>
            <Routes fallback=|| view! { <p>"404 页面未找到"</p> }>
                <Route path=path!("/") view=Home/>
                <ParentRoute path=path!("/dashboard") view=DataLoader>
                    <Route path=path!("/") view=Dashboard/>
                </ParentRoute>
                <Route path=path!("/about") view=About/>
            </Routes>
        </Router>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(Exercise);
}
