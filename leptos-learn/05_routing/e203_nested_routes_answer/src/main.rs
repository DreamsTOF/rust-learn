// ============================================================
// Exercise 203 - Answer: Nested Routes
// ============================================================

use leptos::prelude::*;
use leptos_router::path;
use leptos_router::components::{Router, Routes, Route, ParentRoute, Outlet, A};

#[component]
fn ParentLayout() -> impl IntoView {
    view! {
        <div style="border: 2px solid #4CAF50; padding: 1em; border-radius: 8px;">
            <h2>"父布局"</h2>
            <p>"这是共享的父布局框架"</p>
            <hr/>
            <Outlet/>
        </div>
    }
}

#[component]
fn ChildA() -> impl IntoView {
    view! { <p>"子页面 A 的内容"</p> }
}

#[component]
fn ChildB() -> impl IntoView {
    view! { <p>"子页面 B 的内容"</p> }
}

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <Router>
            <nav>
                <A href="/parent/a">"子页面 A"</A>
                " | "
                <A href="/parent/b">"子页面 B"</A>
            </nav>
            <main>
                <Routes fallback=|| "页面未找到">
                    <ParentRoute path=path!("/parent") view=ParentLayout>
                        <Route path=path!("/a") view=ChildA/>
                        <Route path=path!("/b") view=ChildB/>
                    </ParentRoute>
                </Routes>
            </main>
        </Router>
    }
}

fn main() {
    mount_to_body(Exercise);
}
