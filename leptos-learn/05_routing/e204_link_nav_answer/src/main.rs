// ============================================================
// Exercise 204 - Answer: Declarative Navigation Links
// ============================================================

use leptos::prelude::*;
use leptos_router::path;
use leptos_router::components::{Router, Routes, Route, A};

#[component]
fn Home() -> impl IntoView {
    view! { <h2>"首页"</h2><p>"欢迎来到我们的网站"</p> }
}

#[component]
fn Products() -> impl IntoView {
    view! { <h2>"产品中心"</h2><p>"查看我们的产品"</p> }
}

#[component]
fn Contact() -> impl IntoView {
    view! { <h2>"联系我们"</h2><p>"通过邮件或电话联系我们"</p> }
}

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <Router>
            <nav>
                <ul>
                    <li><A href="/">"首页"</A></li>
                    <li><A href="/products">"产品"</A></li>
                    <li><A href="/contact">"联系"</A></li>
                </ul>
            </nav>
            <main>
                <Routes fallback=|| "页面未找到">
                    <Route path=path!("/") view=Home/>
                    <Route path=path!("/products") view=Products/>
                    <Route path=path!("/contact") view=Contact/>
                </Routes>
            </main>
        </Router>
    }
}

fn main() {
    mount_to_body(Exercise);
}
