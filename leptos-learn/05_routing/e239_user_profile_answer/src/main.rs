// ============================================================
// Exercise 239 - Answer: User Profile
// ============================================================

use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::path;

#[component]
fn ProfileLayout() -> impl IntoView {
    view! {
        <div class="profile-layout">
            <div class="profile-header">
                <h1>"rust_dev"</h1>
                <p>"rust@example.com"</p>
            </div>
            <nav class="profile-tabs">
                <A href="/profile/">"个人资料"</A>
                <A href="/profile/portfolio">"作品集"</A>
                <A href="/profile/settings">"设置"</A>
            </nav>
            <main class="profile-content"><Outlet/></main>
        </div>
    }
}

#[component]
fn ProfileOverview() -> impl IntoView {
    view! {
        <div class="profile-overview">
            <h2>"个人资料"</h2>
            <p>"用户名: rust_dev"</p>
            <p>"邮箱: rust@example.com"</p>
            <p>"注册时间: 2024-01-15"</p>
            <p>"个人简介: 热爱 Rust 编程，专注于 Web 开发。"</p>
        </div>
    }
}

#[component]
fn Portfolio() -> impl IntoView {
    let projects = vec![
        ("Leptos 商城", "基于 Leptos 的全栈电商应用", "Rust, WASM"),
        ("Todo App", "简洁的待办事项管理工具", "Leptos, Signals"),
        ("博客系统", "支持 Markdown 的个人博客", "Rust, SSR"),
        ("数据分析仪表盘", "实时数据可视化面板", "Leptos, Charts"),
    ];
    view! {
        <div>
            <h2>"作品集"</h2>
            <div class="project-grid">
                {projects.iter().map(|(name, desc, tags)| view! {
                    <div class="project-card">
                        <h3>{*name}</h3>
                        <p>{*desc}</p>
                        <span>{*tags}</span>
                    </div>
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}

#[component]
fn Settings() -> impl IntoView {
    view! {
        <div>
            <h2>"账户设置"</h2>
            <form>
                <div><label>"昵称:" <input type="text" value="rust_dev"/></label></div>
                <div><label>"简介:" <textarea>"热爱 Rust 编程。"</textarea></label></div>
                <div><label>"主题:"
                    <select><option>"浅色"</option><option>"深色"</option></select>
                </label></div>
                <button>"保存修改"</button>
            </form>
        </div>
    }
}

#[component]
fn App() -> impl IntoView {
    view! {
        <Router>
            <Routes fallback=|| "404 - 页面未找到">
                <ParentRoute path=path!("profile") view=ProfileLayout>
                    <Route path=path!("") view=ProfileOverview/>
                    <Route path=path!("portfolio") view=Portfolio/>
                    <Route path=path!("settings") view=Settings/>
                </ParentRoute>
            </Routes>
        </Router>
    }
}

fn main() {
    mount_to_body(App);
}
