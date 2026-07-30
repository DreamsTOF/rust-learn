// ============================================================
// 练习 e227: lazy_route — 路由懒加载
//
// 目标: 使用 <Suspense> + LocalResource 实现路由组件的懒加载
//
// 难度: ⭐⭐⭐
// 核心知识点: LocalResource、Suspense、异步加载、fallback 状态
// ============================================================

use leptos::prelude::*;
use leptos_router::components::{Router, Routes, Route, A};
use leptos_router::path;

// 模拟懒加载数据（例如：按需加载的组件数据）
async fn fetch_lazy_data() -> String {
    // 模拟异步 API 调用
    "这是通过懒加载获取的数据".to_string()
}

// 懒加载的首页组件
// TODO: 使用 LocalResource 和 Suspense 实现数据懒加载
#[component]
fn LazyHome() -> impl IntoView {
    // === 步骤 1 ——————————————————————————————————————————
    // TODO: 创建 LocalResource，调用 fetch_lazy_data()
    let data = LocalResource::new(|| async { fetch_lazy_data().await });

    view! {
        <Suspense fallback=|| view! { <p>"正在加载路由..."</p> }>
            <h2>"懒加载页面"</h2>
            <p>{data.map(|d| d.clone())}</p>
        </Suspense>
    }
}

// 立即加载的普通组件
#[component]
fn FastAbout() -> impl IntoView {
    view! {
        <h2>"关于"</h2>
        <p>"这个页面是立即加载的"</p>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <Router>
            <nav>
                <A href="/">"首页（懒加载）"</A>
                <A href="/about">"关于"</A>
            </nav>
            <main>
                <Routes fallback=|| "页面未找到">
                    <Route path=path!("/") view=LazyHome/>
                    <Route path=path!("/about") view=FastAbout/>
                </Routes>
            </main>
        </Router>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(Exercise);
}

// ============================================================
// 参考答案 (思考后再看!)
// ============================================================
// <details>
// <summary>点击展开答案</summary>
//
// ### 代码说明
// - `LocalResource::new(|| async { ... })` 创建客户端异步资源
// - `<Suspense fallback=|| view! { ... }>` 包裹懒加载区域
// - 资源未就绪时显示 fallback，就绪后渲染实际内容
// - `data.map(|d| d.clone())` 从 Resource 中读取 `Option<T>`
//
// ### 知识点
// - `LocalResource` 适用于纯客户端数据加载（无需 serde）
// - `Suspense` 可以嵌套使用，支持多个资源
// - 真正生产环境中，可用 `Resource` + 服务端函数做 SSR
//
// </details>
