// ============================================================
// 练习 e222: 布局组件接收 Props (layout_props)
//
// 目标: 布局组件接收 props，通过 provide_context 向子路由传递
//       数据，子路由通过 use_context 获取布局数据。
//
// 难度: ⭐⭐
// 核心知识点: 组件 props、provide_context、use_context
//
// TODO: 按照注释提示补全代码
// ============================================================

use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::path;

// === 步骤 1 ——————————————————————————————————————————
// TODO: 定义 LayoutData 结构体（需要 #[derive(Clone)]）
//       包含 title: String 和 description: String 两个字段
// TODO: 定义 UserInfo 结构体（需要 #[derive(Clone)]）
//       包含 name: String 和 role: String 两个字段

// === 步骤 2 ——————————————————————————————————————————
// TODO: 创建 AppLayout 组件，接收 title: String 参数
//       使用 provide_context 传递 LayoutData 给子路由
//       布局中包含标题区域 + <Outlet/>

// === 步骤 3 ——————————————————————————————————————————
// TODO: 创建两个子路由组件 Profile 和 Settings
//       使用 use_context 获取 LayoutData 并渲染
//       Profile 额外获取 UserInfo 并显示

#[component]
fn Profile() -> impl IntoView {
    // TODO: 使用 use_context::<LayoutData>() 和 use_context::<UserInfo>() 获取数据
    view! {
        // TODO: 显示用户信息
    }
}

#[component]
fn Settings() -> impl IntoView {
    // TODO: 使用 use_context::<LayoutData>() 获取数据
    view! {
        // TODO: 显示设置页面
    }
}

#[component]
fn Exercise() -> impl IntoView {
    // === 步骤 4 ——————————————————————————————————————————
    // TODO: 在 <Router> 中创建路由结构
    // 使用 ParentRoute 作为布局路由，view 使用闭包传递 title prop
    // 例如: view=move || view! { <AppLayout title="用户中心"/> }
    // 子路由 path="profile" 渲染 Profile
    // 子路由 path="settings" 渲染 Settings

    view! {
        // TODO
    }
}

fn main() {
    mount_to_body(Exercise);
}
