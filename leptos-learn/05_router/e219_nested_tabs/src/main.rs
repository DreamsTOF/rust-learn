// ============================================================
// 练习 219: nested_tabs — Tab 切换绑定子路由
//
// 目标: 使用 <A> 作为 Tab 按钮实现 URL 同步切换，刷新保留 Tab 状态
//
// 难度: ⭐⭐⭐
// 核心知识点: <A> 组件 Tab 导航, URL 同步
//
// TODO: 按照注释提示补全代码
// ============================================================

use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::path;

// === 步骤 1 ——————————————————————————————————————————
// TODO: TabLayout 布局组件
// 提示: 使用 <A> 组件渲染 Tab 导航按钮
#[component]
fn TabLayout() -> impl IntoView {
    view! {
        <nav>
            // TODO: 使用 <A> 组件创建三个 Tab: "个人信息", "设置", "通知"
            <A href="/tabs/profile">"个人信息"</A>
            <A href="/tabs/settings">"设置"</A>
            <A href="/tabs/notifications">"通知"</A>
        </nav>
        // TODO: 在这里放置 Outlet 组件
        <Outlet/>
    }
}

// TODO: 创建 Profile 组件，显示个人信息
#[component]
fn Profile() -> impl IntoView {
    view! {
        <h3>"个人信息"</h3>
        <p>"姓名: 张三"</p>
        <p>"邮箱: zhang@example.com"</p>
    }
}

// TODO: 创建 Settings 组件，显示设置选项
#[component]
fn Settings() -> impl IntoView {
    view! {
        <h3>"设置"</h3>
        <p>"主题: 浅色"</p>
        <p>"语言: 中文"</p>
    }
}

// TODO: 创建 Notifications 组件，显示通知
#[component]
fn Notifications() -> impl IntoView {
    view! {
        <h3>"通知"</h3>
        <p>"您有 3 条未读消息"</p>
    }
}

fn main() {
    mount_to_body(|| view! {
        // TODO: 配置路由器，使用 ParentRoute 定义 Tab 导航路由
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
    });
}
