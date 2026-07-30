// ============================================================
// 练习 e239: user_profile — 用户主页
//
// 目标: 实现用户个人主页、作品集和设置的嵌套路由
//
// 难度: ⭐⭐
// 核心知识点: 嵌套路由, 标签页导航, Outlet
//
// TODO: 理解嵌套路由与标签页 UI 的对应关系
// ============================================================

use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::path;

// === 步骤 1 — ProfileLayout: 用户主页布局 ———————————
// TODO: A 组件自动设置 aria-current，可用 CSS 实现标签页切换效果
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

// === 步骤 2 — Portfolio: 作品集网格 ——————————————
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

// === 步骤 3 — Settings: 设置表单 ———————————————
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

// === 步骤 4 — App: 路由配置 ——————————————————————
// TODO: 尝试添加"收藏"标签页
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

// ============================================================
// 参考答案 (思考后再看!)
// ============================================================
// <details>
// <summary>点击展开答案</summary>
//
// ### 添加"收藏"标签
// ```rust
// #[component]
// fn Favorites() -> impl IntoView {
//     view! {
//         <div>
//             <h2>"收藏"</h2>
//             <p>"你收藏的项目将显示在这里。"</p>
//         </div>
//     }
// }
// // ProfileLayout 中添加:
// // <A href="/profile/favorites" active_class="active">"收藏"</A>
// // App 路由中添加:
// // <Route path=path!("favorites") view=Favorites/>
// ```
//
// ### 知识点
// - 嵌套路由配合 <Outlet> 实现用户主页的标签页切换
// - <A> 自动设置 aria-current 便于 CSS 高亮当前标签
// - Route path=path!("") 匹配 /profile/ 作为默认页
//
// </details>
