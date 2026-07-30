// ============================================================
// 练习 e221: 多级嵌套布局 (nested_layout)
//
// 目标: 创建包含 header + sidebar + content 的多级嵌套布局，
//       使用 <Outlet/> 在布局组件中渲染子路由内容。
//
// 难度: ⭐⭐
// 核心知识点: ParentRoute、<Outlet/>、布局组件封装
//
// TODO: 按照注释提示补全代码
// ============================================================

use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::path;

// === 步骤 1 ——————————————————————————————————————————
// TODO: 创建 AppLayout 布局组件，包含：
//       1. header：应用标题 + 导航链接（Home / About / Contact）
//       2. flex 容器：左侧 sidebar（菜单列表）+ 右侧 <main>（<Outlet/>）
//       3. footer：版权信息
// 提示: <Outlet/> 会渲染匹配到的子路由组件
// 提示: 需要添加 ParentRoute 和 Outlet 等组件的导入

// === 步骤 2 ——————————————————————————————————————————
// TODO: 创建 Home 页面组件，显示 "Home Page"
// TODO: 创建 About 页面组件，显示 "About Page"
// TODO: 创建 Contact 页面组件，显示 "Contact Page"

#[component]
fn Home() -> impl IntoView {
    view! {
        // TODO: 显示首页标题和欢迎文字
    }
}

#[component]
fn About() -> impl IntoView {
    view! {
        // TODO: 显示关于页面标题和内容
    }
}

#[component]
fn Contact() -> impl IntoView {
    view! {
        // TODO: 显示联系页面标题和内容
    }
}

#[component]
fn Exercise() -> impl IntoView {
    // === 步骤 3 ——————————————————————————————————————————
    // TODO: 使用 <Router> 包裹整个应用
    // TODO: 在 <Routes fallback=|| "Page not found"> 中配置嵌套路由：
    //       - <ParentRoute path=path!("/") view=AppLayout> 作为父路由
    //         - 子路由 path=path!("") 渲染 Home
    //         - 子路由 path=path!("about") 渲染 About
    //         - 子路由 path=path!("contact") 渲染 Contact

    view! {
        // TODO
    }
}

fn main() {
    mount_to_body(Exercise);
}
