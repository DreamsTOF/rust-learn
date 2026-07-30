// ============================================================
// Exercise 237 - Answer: Dashboard Routes
// ============================================================

use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::path;

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
