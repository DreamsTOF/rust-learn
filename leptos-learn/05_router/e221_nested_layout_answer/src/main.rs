// ============================================================
// Exercise 221 - Answer (nested_layout)
// ============================================================

use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::path;

#[component]
fn AppLayout() -> impl IntoView {
    view! {
        <div style="display:flex;flex-direction:column;min-height:300px;font-family:sans-serif;">
            <header style="background:#4CAF50;color:white;padding:12px 16px;">
                <h1 style="margin:0;">"My App"</h1>
                <nav style="margin-top:8px;">
                    <span style="color:white;margin-right:12px;"><A href="/">"Home"</A></span>
                    <span style="color:white;margin-right:12px;"><A href="/about">"About"</A></span>
                    <span style="color:white;"><A href="/contact">"Contact"</A></span>
                </nav>
            </header>
            <div style="display:flex;flex:1;">
                <aside style="width:180px;background:#f5f5f5;padding:16px;border-right:1px solid #ddd;">
                    <h3>"Sidebar"</h3>
                    <ul style="list-style:none;padding:0;">
                        <li><A href="/">"Dashboard"</A></li>
                        <li><A href="/about">"About Us"</A></li>
                        <li><A href="/contact">"Contact"</A></li>
                    </ul>
                </aside>
                <main style="flex:1;padding:16px;">
                    <Outlet/>
                </main>
            </div>
            <footer style="background:#333;color:white;padding:8px 16px;text-align:center;">
                <p style="margin:0;">"© 2026 My App"</p>
            </footer>
        </div>
    }
}

#[component]
fn Home() -> impl IntoView {
    view! { <h2>"Home Page"</h2><p>"Welcome to the home page!"</p> }
}

#[component]
fn About() -> impl IntoView {
    view! { <h2>"About Page"</h2><p>"This is the about page."</p> }
}

#[component]
fn Contact() -> impl IntoView {
    view! { <h2>"Contact Page"</h2><p>"Contact us at contact@example.com"</p> }
}

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <Router>
            <Routes fallback=|| "Page not found">
                <ParentRoute path=path!("/") view=AppLayout>
                    <Route path=path!("") view=Home/>
                    <Route path=path!("about") view=About/>
                    <Route path=path!("contact") view=Contact/>
                </ParentRoute>
            </Routes>
        </Router>
    }
}

fn main() {
    mount_to_body(Exercise);
}
