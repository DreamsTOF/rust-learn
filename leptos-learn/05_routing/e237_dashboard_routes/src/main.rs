// ============================================================
// 练习 e237: dashboard_routes — 后台路由
//
// 目标: 实现后台管理系统的侧边栏 + 内容区布局路由
//
// 难度: ⭐⭐
// 核心知识点: ParentRoute, Outlet, A, 嵌套布局
//
// TODO: 研究侧边栏导航与 <Outlet/> 的配合方式
// ============================================================

use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::path;

// === 步骤 1 — DashboardLayout: 侧边栏 + 内容区 ——————
// TODO: A 组件自动设置 aria-current，可用 CSS [aria-current="page"] 实现高亮
#[component]
fn DashboardLayout() -> impl IntoView {
    view! {
        <div class="dashboard-layout">
            <aside class="sidebar">
                <h2>"后台管理"</h2>
                <nav>
                    <A href="/dashboard/">"概览"</A>
                    <A href="/dashboard/users">"用户"</A>
                    <A href="/dashboard/settings">"设置"</A>
                </nav>
            </aside>
            <main class="content"><Outlet/></main>
        </div>
    }
}

#[component]
fn Overview() -> impl IntoView {
    view! {
        <div>
            <h1>"控制台概览"</h1>
            <div class="stats">
                <div class="stat-card">"用户数: 1,234"</div>
                <div class="stat-card">"订单数: 567"</div>
                <div class="stat-card">"营收: ¥89,012"</div>
            </div>
        </div>
    }
}

// === 步骤 2 — UserList: 数据表格展示 ———————————————
#[component]
fn UserList() -> impl IntoView {
    let users = vec![
        (1, "alice", "alice@example.com", "管理员"),
        (2, "bob", "bob@example.com", "编辑"),
        (3, "charlie", "charlie@example.com", "用户"),
    ];
    view! {
        <div>
            <h1>"用户管理"</h1>
            <table>
                <tr><th>ID</th><th>用户名</th><th>邮箱</th><th>角色</th></tr>
                {users.iter().map(|(id, name, email, role)| view! {
                    <tr><td>{*id}</td><td>{*name}</td><td>{*email}</td><td>{*role}</td></tr>
                }).collect::<Vec<_>>()}
            </table>
        </div>
    }
}

// === 步骤 3 — Settings: 设置表单 UI —————————————————
#[component]
fn Settings() -> impl IntoView {
    view! {
        <div>
            <h1>"系统设置"</h1>
            <form>
                <div><label>"网站名称:" <input type="text" value="我的站点"/></label></div>
                <div><label>"语言:"
                    <select><option>"中文"</option><option>"English"</option></select>
                </label></div>
                <button>"保存设置"</button>
            </form>
        </div>
    }
}

// === 步骤 4 — App: 路由配置 ——————————————————————————
// TODO: 尝试在侧边栏添加"订单"导航链接
#[component]
fn App() -> impl IntoView {
    view! {
        <Router>
            <Routes fallback=|| "404 - 页面未找到">
                <Route path=path!("") view=move || view! { <Redirect path="/dashboard/"/> }/>
                <ParentRoute path=path!("dashboard") view=DashboardLayout>
                    <Route path=path!("") view=Overview/>
                    <Route path=path!("users") view=UserList/>
                    <Route path=path!("settings") view=Settings/>
                </ParentRoute>
            </Routes>
        </Router>
    }
}

fn main() {
    mount_to_body(App);
}

// ============================================================
// 参考答案 (思考后再看!)
// ============================================================
// <details>
// <summary>点击展开答案</summary>
//
// ### 添加"订单"页
// ```rust
// // 1. 创建组件
// #[component]
// fn Orders() -> impl IntoView {
//     view! { <div><h1>"订单管理"</h1><p>"订单列表..."</p></div> }
// }
// // 2. DashboardLayout 侧边栏添加:
// // <A href="/dashboard/orders" active_class="active">"订单"</A>
// // 3. App 路由添加:
// // <Route path=path!("orders") view=Orders/>
// ```
//
// ### 知识点
// - A 组件自动设置 `aria-current="page"`，可通过 CSS 实现导航高亮
// - `Redirect` 组件用于根路径重定向到默认仪表盘
// - 侧边栏 + Outlet 是后台管理系统经典布局模式
//
// </details>
