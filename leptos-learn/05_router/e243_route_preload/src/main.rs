// ============================================================
// 练习 e243: route_preload — 路由预加载数据
//
// 目标: 使用 LocalResource + Suspense 在路由渲染前预加载数据，
//       通过 provide_context 将 Resource 传给子路由
//
// 难度: ⭐⭐⭐
// 核心知识点: LocalResource、Suspense、provide_context、预加载
// ============================================================

use leptos::prelude::*;
use leptos_router::components::{Router, Routes, Route, ParentRoute, A};
use leptos_router::components::Outlet;
use leptos_router::path;

// 定义预加载数据的类型
#[derive(Clone, Debug)]
struct DashboardData {
    title: String,
    content: String,
}

// 模拟从 API 加载数据
// TODO: 实现异步数据加载函数
async fn load_dashboard_data() -> DashboardData {
    DashboardData {
        title: "仪表盘数据".into(),
        content: "这是一条在路由渲染前预加载的数据。\n\nLocalResource 会在组件第一次创建时自动触发异步加载，子组件通过 .map() 获取数据。".into(),
    }
}

// 数据加载层 — 父路由组件，提前触发数据加载
// TODO: 创建 LocalResource，provide_context 给子路由，用 <Suspense> 包裹 <Outlet/>
#[component]
fn DataLoader() -> impl IntoView {
    // ★ 创建 LocalResource 在路由进入时立即加载数据
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

// 仪表盘 — 消费预加载的数据
// TODO: 通过 use_context 获取 LocalResource，使用 .map() 读取数据
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

// 首页 — 无数据预加载
#[component]
fn Home() -> impl IntoView {
    view! {
        <div>
            <h2>"🏠 首页"</h2>
            <p>"点击"仪表盘"链接观察预加载效果"</p>
        </div>
    }
}

// 关于页面 — 无数据预加载
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

// ============================================================
// 参考答案 (思考后再看!)
// ============================================================
// <details>
// <summary>点击展开答案</summary>
//
// ### 预加载模式
// DataLoader 创建 LocalResource 并 provide_context，<Suspense> 包裹 <Outlet/>
// 子路由通过 use_context 获取 LocalResource 并 .map() 读取
//
// ### 关键点
// - LocalResource 用于纯客户端数据，无需 serde
// - Suspense 处理加载状态
// - context 传递 Resource 本身而非值，保持响应式
//
// </details>
