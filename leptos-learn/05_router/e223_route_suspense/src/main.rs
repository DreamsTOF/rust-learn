// ============================================================
// 练习 e223: 路由 Suspense (route_suspense)
//
// 目标: 使用 <Suspense/> 包裹路由内容，fallback 显示加载态，
//       结合 LocalResource 实现异步数据加载。
//
// 难度: ⭐⭐⭐
// 核心知识点: LocalResource、<Suspense/>、fallback 加载态
//
// TODO: 按照注释提示补全代码
// ============================================================

use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::path;

// === 步骤 1 ——————————————————————————————————————————
// TODO: 创建异步函数 async fn fetch_user_profile() -> String
//       模拟从 API 获取用户名称（返回 "Alice"）
// 提示: 异步函数是 async 的，LocalResource 会处理异步状态

async fn fetch_user_profile() -> String {
    // TODO: 返回 "Alice"
}

// === 步骤 2 ——————————————————————————————————————————
// TODO: 创建 Home 页面组件
//       使用 LocalResource::new(|| async { ... }) 调用 fetch_user_profile
//       使用 <Suspense fallback=|| view! { ... }> 包裹内容
//       加载时显示 "Loading user profile..."
//       加载完成后显示用户名

#[component]
fn Home() -> impl IntoView {
    // TODO: 创建 LocalResource
    // TODO: 返回 Suspense 包裹的视图
    view! {
        // TODO
    }
}

// === 步骤 3 ——————————————————————————————————————————
// TODO: 创建 About 页面组件（非异步，直接显示）

#[component]
fn About() -> impl IntoView {
    view! {
        // TODO
    }
}

#[component]
fn Exercise() -> impl IntoView {
    // === 步骤 4 ——————————————————————————————————————————
    // TODO: 配置路由
    //       - "/" 路径渲染 Home（含 Suspense）
    //       - "/about" 路径渲染 About

    view! {
        <Router>
            // TODO: 导航链接 + Routes
        </Router>
    }
}

fn main() {
    mount_to_body(Exercise);
}
