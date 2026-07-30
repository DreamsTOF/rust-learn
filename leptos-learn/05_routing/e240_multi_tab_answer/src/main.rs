// ============================================================
// Exercise 240 - Answer: Multi-Tab
// ============================================================

use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::path;

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
                        <p>"发现更多" {*cat} "内容"</p>
                    </div>
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}

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
