// ============================================================
// 练习 e225: 导航激活高亮 (active_nav_highlight)
//
// 目标: 使用 <A> 的 class:active 实现自定义激活样式
//       （下划线 / 背景色 / 字体加粗），并实现父路由
//       激活时子路由链接的高亮。
//
// 难度: ⭐⭐
// 核心知识点: <A class:active>、use_location、CSS 样式
//
// TODO: 按照注释提示补全代码
// ============================================================

use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::path;

// === 步骤 1 ——————————————————————————————————————————
// TODO: 创建页面组件 Dashboard、Analytics、Reports、Settings、Profile
//       每个组件只包含 <h2> 标题加 <p> 简述即可

// === 步骤 2 ——————————————————————————————————————————
// TODO: 创建 SidebarLayout 布局组件，包含：
//       - 左侧 sidebar：导航菜单列表
//       - 右侧 <main>：<Outlet/>
// 提示: 导航菜单使用 <A> 配合 class:active 实现自动高亮
// 提示: 父路由 Dashboard 需要手动检测 URL 以支持子路由高亮
//       可使用 use_location().pathname.get().starts_with("/dashboard")

// === 步骤 3 ——————————————————————————————————————————
// TODO: 在 Exercise 中添加 <style> 定义 .active 样式：
//       - font-weight: bold
//       - background-color: #e3f2fd
//       - color: #1976d2
//       - border-right: 3px solid #1976d2

#[component]
fn Exercise() -> impl IntoView {
    // TODO: 配置路由结构
    // 使用 ParentRoute 作为 SidebarLayout 的容器
    // 子路由包括 dashboard, dashboard/analytics, dashboard/reports, settings, profile

    view! {
        // TODO
    }
}

fn main() {
    mount_to_body(Exercise);
}
