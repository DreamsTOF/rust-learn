// ============================================================
// 练习 e240: multi_tab — 多标签页
//
// 目标: 实现浏览器 Tab 风格的多标签页路由
//
// 难度: ⭐⭐⭐
// 核心知识点: 路由切换, 多 Tab 布局, 选中态同步
//
// TODO: 理解 Tab 布局与路由的对应关系
// ============================================================

use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::path;

// === 步骤 1 — TabLayout: Tab 栏布局 ——————————————
// TODO: A 组件自动设置 aria-current，可用 CSS 实现 Tab 切换效果
#[component]
fn TabLayout() -> impl IntoView {
    view! {
        <div class="tab-layout">
            <div class="tab-bar">
                <A href="/tabs/home">"首页"</A>
                <A href="/tabs/explore">"探索"</A>
                <A href="/tabs/notifications">"通知"</A>
                <A href="/tabs/profile">"我的"</A>
            </div>
            <div class="tab-panel"><Outlet/></div>
        </div>
    }
}

#[component]
fn TabHome() -> impl IntoView {
    view! {
        <div class="tab-content">
            <h2>"首页"</h2>
            <p>"欢迎使用多标签页应用！点击上方 Tab 切换页面。"</p>
            <div class="feed">
                <div class="feed-item">"动态: 今天天气真好！"</div>
                <div class="feed-item">"动态: 发布了新文章"</div>
                <div class="feed-item">"动态: 完成了一个项目"</div>
            </div>
        </div>
    }
}

// === 步骤 2 — TabExplore: 探索页面 ——————————————
#[component]
fn TabExplore() -> impl IntoView {
    let categories = vec!["推荐", "热门", "最新", "科技", "设计", "教程"];
    view! {
        <div>
            <h2>"探索"</h2>
            <div class="explore-grid">
                {categories.iter().map(|cat| view! {
                    <div class="explore-card">
                        <h3>{*cat}</h3>
                        <p> "发现更多" {*cat} "内容"</p>
                    </div>
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}

// === 步骤 3 — TabNotifications: 通知列表 ——————————
#[component]
fn TabNotifications() -> impl IntoView {
    let notifs = vec![
        ("用户 张三 赞了你的帖子", "3 分钟前"),
        ("用户 李四 评论了你的文章", "1 小时前"),
        ("系统: 你的账号已升级", "昨天"),
        ("你的项目被加入收藏", "3 天前"),
    ];
    view! {
        <div>
            <h2>"通知"</h2>
            <ul class="notif-list">
                {notifs.iter().map(|(msg, time)| view! {
                    <li class="notif-item">
                        <span>{*msg}</span>
                        <span>{*time}</span>
                    </li>
                }).collect::<Vec<_>>()}
            </ul>
        </div>
    }
}

// === 步骤 4 — TabProfile: 个人中心 ————————————————
#[component]
fn TabProfile() -> impl IntoView {
    view! {
        <div>
            <h2>"我的"</h2>
            <div class="profile-card">
                <h3>"rust_dev"</h3>
                <p>"rust@example.com"</p>
            </div>
            <ul class="settings-list">
                <li>"我的文件"</li>
                <li>"收藏夹"</li>
                <li>"账户设置"</li>
                <li>"使用帮助"</li>
            </ul>
        </div>
    }
}

// === 步骤 5 — App: 路由配置 ——————————————————————
// TODO: 尝试新增加一个"消息" Tab
#[component]
fn App() -> impl IntoView {
    view! {
        <Router>
            <Routes fallback=|| "404 - 页面未找到">
                <Route path=path!("") view=move || view! { <Redirect path="/tabs/home"/> }/>
                <ParentRoute path=path!("tabs") view=TabLayout>
                    <Route path=path!("home") view=TabHome/>
                    <Route path=path!("explore") view=TabExplore/>
                    <Route path=path!("notifications") view=TabNotifications/>
                    <Route path=path!("profile") view=TabProfile/>
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
// ### 添加"消息" Tab
// ```rust
// #[component]
// fn TabMessages() -> impl IntoView {
//     view! { <div><h2>"消息"</h2><p>"私信和系统消息。"</p></div> }
// }
// // TabLayout 中添加:
// // <A href="/tabs/messages" active_class="active">"消息"</A>
// // App 路由中添加:
// // <Route path=path!("messages") view=TabMessages/>
// ```
//
// ### 知识点
// - Tab 风格 UI + 路由: 每个 Tab 对应一条独立路由
// - `Redirect` 将根路径重定向到默认 Tab
// - `ParentRoute` 的 Tab 布局 + `Outlet` 实现 Tab 面板切换
// - `<A>` 自动设置 aria-current 便于 CSS 高亮当前 Tab
//
// </details>
