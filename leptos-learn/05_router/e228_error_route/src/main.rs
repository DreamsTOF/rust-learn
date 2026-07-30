// ============================================================
// 练习 e228: error_route — 路由级错误边界
//
// 目标: 使用 <ErrorBoundary/> 包裹路由内容，捕获子组件的错误
//
// 难度: ⭐⭐
// 核心知识点: ErrorBoundary、错误捕获、fallback 显示友好信息
// ============================================================

use leptos::prelude::*;
use leptos_router::components::{Router, Routes, Route, A};
use leptos_router::path;
use std::fmt;

// 自定义错误类型（实现 std::error::Error 才能被 ErrorBoundary 捕获）
#[derive(Debug, Clone)]
struct RouteError(&'static str);

impl fmt::Display for RouteError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for RouteError {}

// 首页
#[component]
fn Home() -> impl IntoView {
    view! {
        <h2>"首页"</h2>
        <p>"欢迎来到首页"</p>
    }
}

// 关于页面
#[component]
fn About() -> impl IntoView {
    view! {
        <h2>"关于"</h2>
        <p>"这是关于页面"</p>
    }
}

// 模拟会出错的组件
// TODO: 让此组件返回 Err，触发 ErrorBoundary 捕获
#[component]
fn ErrorProne() -> impl IntoView {
    // === 步骤 1 ——————————————————————————————————————————
    // TODO: 创建 Result::Err 值，渲染到 view! 中触发 ErrorBoundary
    let data: Result<&'static str, RouteError> = Err(RouteError("模拟的数据库查询失败！请在修复后重试。"));

    view! {
        <h2>"危险页面"</h2>
        <p>{data}</p>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <Router>
            <nav>
                <A href="/">"首页"</A>
                <A href="/about">"关于"</A>
                <A href="/danger">"危险页面"</A>
            </nav>
            <main>
                // === 步骤 2 ——————————————————————————————————————————
                // TODO: 用 <ErrorBoundary> 包裹 <Routes>
                // fallback 接收 errors 并显示友好的错误信息
                <ErrorBoundary fallback=|errors| view! {
                    <div style="color:red;padding:1rem;border:1px solid red;border-radius:4px;">
                        <h2>"出错了！"</h2>
                        {move || {
                            errors
                                .get()
                                .iter()
                                .map(|(_, e)| view! { <p>{e.to_string()}</p> })
                                .collect::<Vec<_>>()
                        }}
                        <A href="/">"返回首页"</A>
                    </div>
                }>
                    <Routes fallback=|| "页面未找到">
                        <Route path=path!("/") view=Home/>
                        <Route path=path!("/about") view=About/>
                        <Route path=path!("/danger") view=ErrorProne/>
                    </Routes>
                </ErrorBoundary>
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
// - `<ErrorBoundary fallback=|errors| view! { ... }>` 捕获子组件的错误
// - 组件返回 `Result::Err` 或渲染 `Result::Err` 值时触发 ErrorBoundary
// - `errors.get()` 返回错误列表，每个元素是 `(id, Box<dyn Error>)`
// - fallback 内可以读取错误信息，显示友好的用户界面
// - 导航到其他路由后 ErrorBoundary 自动重置
//
// ### 知识点
// - `ErrorBoundary` 是 leptos 内置的错误边界组件
// - 错误类型必须实现 `std::error::Error`
// - 路由级别的 ErrorBoundary 应该放在 `<Routes>` 外层
// - `<A>` 链接导航会自动重置 ErrorBoundary 状态
//
// </details>
