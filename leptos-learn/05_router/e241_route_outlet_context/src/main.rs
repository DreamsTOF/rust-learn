// ============================================================
// 练习 e241: route_outlet_context — 通过 Outlet 传递 Context
//
// 目标: 使用 provide_context 在 Layout 中通过 <Outlet/> 传递数据
//       子路由通过 use_context 接收父布局提供的数据
//
// 难度: ⭐⭐⭐
// 核心知识点: Outlet、provide_context、use_context、ParentRoute
// ============================================================

use leptos::prelude::*;
use leptos_router::components::{Router, Routes, Route, ParentRoute, A};
use leptos_router::components::Outlet;
use leptos_router::path;

// 定义将通过 context 共享的数据类型（需要 Clone）
// TODO: 为 User 添加需要的 derive
#[derive(Clone, Debug)]
struct User {
    name: String,
    role: String,
}

// 首页 — 不依赖父布局中的 context
#[component]
fn Home() -> impl IntoView {
    view! {
        <div>
            <h2>"🏠 首页"</h2>
            <p>"欢迎来到路由 context 示例"</p>
        </div>
    }
}

// 仪表盘布局 — 提供 User context 给子路由
// TODO: 创建 User 实例并通过 provide_context 传递给 <Outlet/>
#[component]
fn DashboardLayout() -> impl IntoView {
    let user = User {
        name: "Alice".into(),
        role: "管理员".into(),
    };
    // ★ 将 user 注入组件树，子路由可通过 use_context 访问
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

// 个人资料 — 通过 use_context 获取 User 数据
// TODO: 使用 use_context 获取 DashboardLayout 提供的 User
#[component]
fn Profile() -> impl IntoView {
    let user = use_context::<User>()
        .expect("User context 未提供，请确保在 DashboardLayout 内访问此页面");

    view! {
        <div style="background:#f0f8ff;padding:12px;border-radius:4px;margin-top:8px;">
            <h3>"👤 个人资料"</h3>
            <p>"姓名: " {user.name.clone()}</p>
            <p>"角色: " {user.role.clone()}</p>
        </div>
    }
}

// 设置 — 同样通过 use_context 获取 User
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

// ============================================================
// 参考答案 (思考后再看!)
// ============================================================
// <details>
// <summary>点击展开答案</summary>
//
// ### 关键代码
// 1. 定义 context 类型：`#[derive(Clone)] struct User { ... }`
// 2. 父布局通过 `provide_context(user)` 注入
// 3. 子路由通过 `use_context::<User>()` 读取
//
// ### 知识点
// - context 按类型（TypeId）匹配，无需字符串 key
// - Outlet 保留父组件的响应式作用域，子路由继承 scope
// - ParentRoute + Outlet 实现嵌套布局
//
// </details>
