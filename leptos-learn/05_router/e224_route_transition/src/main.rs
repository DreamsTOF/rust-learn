// ============================================================
// 练习 e224: 路由过渡效果 (route_transition)
//
// 目标: 在路由切换时添加平滑的过渡效果，利用 CSS transition
//       实现旧内容保留 / 新内容淡入。
//
// 难度: ⭐⭐⭐
// 核心知识点: set_is_routing、CSS transition、路由过渡动画
//
// TODO: 按照注释提示补全代码
// ============================================================

use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::path;

// === 步骤 1 ——————————————————————————————————————————
// TODO: 创建三个页面组件 Home、About、Contact
//       每个组件用 <h2> 和 <p> 渲染简单内容

#[component]
fn Home() -> impl IntoView {
    view! {
        // TODO
    }
}

#[component]
fn About() -> impl IntoView {
    view! {
        // TODO
    }
}

#[component]
fn Contact() -> impl IntoView {
    view! {
        // TODO
    }
}

#[component]
fn Exercise() -> impl IntoView {
    // === 步骤 2 ——————————————————————————————————————————
    // TODO: 创建 is_routing 信号 (signal(false))
    // TODO: 将 set_is_routing 传给 <Router set_is_routing=...>
    // TODO: 根据 is_routing 的值动态设置容器 class
    //       例如: 导航中时 class="route-container exiting"
    //             导航完成时 class="route-container entering"
    //             默认 class="route-container"
    // 提示: CSS 已预置在 index.html 中
    // 提示: 可使用 Effect 或 Signal::derive 管理 class

    let (is_routing, set_is_routing) = signal(false);

    view! {
        // TODO: 使用 <Router set_is_routing=set_is_routing>
        // TODO: 添加导航链接 (Home / About / Contact)
        // TODO: <Routes fallback=|| "Page not found">
        //       配置三个路由："/" "/about" "/contact"
    }
}

fn main() {
    mount_to_body(Exercise);
}
