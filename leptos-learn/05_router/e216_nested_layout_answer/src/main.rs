// ============================================================
// Exercise 216 - Answer
// nested_layout — 嵌套路由布局
// ============================================================

use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::path;

#[component]
fn Layout() -> impl IntoView {
    view! {
        <nav>
            <a href="/">"首页"</a>
            <a href="/about">"关于"</a>
        </nav>
        <Outlet/>
    }
}

#[component]
fn Home() -> impl IntoView {
    view! {
        <h2>"首页"</h2>
        <p>"欢迎来到嵌套路由示例"</p>
    }
}

#[component]
fn About() -> impl IntoView {
    view! {
        <h2>"关于页面"</h2>
        <p>"这是嵌套路由布局中的子路由页面"</p>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <Router>
            <main>
                <Routes fallback=|| "页面未找到">
                    <ParentRoute path=path!("/") view=Layout>
                        <Route path=path!("/") view=Home/>
                        <Route path=path!("/about") view=About/>
                    </ParentRoute>
                </Routes>
            </main>
        </Router>
    }
}

fn main() {
    mount_to_body(Exercise);
}
