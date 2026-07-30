// ============================================================
// Exercise 219 - Answer
// nested_tabs — Tab 切换绑定子路由
// ============================================================

use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::path;

#[component]
fn TabLayout() -> impl IntoView {
    view! {
        <nav>
            <A href="/tabs/profile">"个人信息"</A>
            <A href="/tabs/settings">"设置"</A>
            <A href="/tabs/notifications">"通知"</A>
        </nav>
        <Outlet/>
    }
}

#[component]
fn Profile() -> impl IntoView {
    view! {
        <h3>"个人信息"</h3>
        <p>"姓名: 张三"</p>
        <p>"邮箱: zhang@example.com"</p>
    }
}

#[component]
fn Settings() -> impl IntoView {
    view! {
        <h3>"设置"</h3>
        <p>"主题: 浅色"</p>
        <p>"语言: 中文"</p>
    }
}

#[component]
fn Notifications() -> impl IntoView {
    view! {
        <h3>"通知"</h3>
        <p>"您有 3 条未读消息"</p>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <Router>
            <main>
                <h2>"用户中心 (Tab 导航)"</h2>
                <Routes fallback=|| "页面未找到">
                    <ParentRoute path=path!("/tabs") view=TabLayout>
                        <Route path=path!("/tabs/profile") view=Profile/>
                        <Route path=path!("/tabs/settings") view=Settings/>
                        <Route path=path!("/tabs/notifications") view=Notifications/>
                    </ParentRoute>
                </Routes>
            </main>
        </Router>
    }
}

fn main() {
    mount_to_body(Exercise);
}
