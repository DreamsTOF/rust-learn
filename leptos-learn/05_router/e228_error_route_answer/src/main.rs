// ============================================================
// Exercise 228 — Answer: error_route
// ============================================================

use leptos::prelude::*;
use leptos_router::components::{Router, Routes, Route, A};
use leptos_router::path;
use std::fmt;

#[derive(Debug, Clone)]
struct RouteError(&'static str);

impl fmt::Display for RouteError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for RouteError {}

#[component]
fn Home() -> impl IntoView {
    view! {
        <h2>"首页"</h2>
        <p>"欢迎来到首页"</p>
    }
}

#[component]
fn About() -> impl IntoView {
    view! {
        <h2>"关于"</h2>
        <p>"这是关于页面"</p>
    }
}

#[component]
fn ErrorProne() -> impl IntoView {
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
