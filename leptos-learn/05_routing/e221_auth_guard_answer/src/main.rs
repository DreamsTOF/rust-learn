// ============================================================
// Exercise 221 - Answer (auth_guard)
// ============================================================

use leptos::prelude::*;
use leptos_router::components::{Router, Routes, Route, ParentRoute, A, Redirect};
use leptos_router::components::Outlet;
use leptos_router::path;

/// Simulate checking auth status
async fn check_auth() -> bool {
    true // Simulate authenticated user
}

/// Home page — public, no auth needed
#[component]
fn Home() -> impl IntoView {
    view! {
        <div>
            <h2>"Home"</h2>
            <p>"This is a public page. Anyone can view it."</p>
        </div>
    }
}

/// Login page — public, for unauthenticated users
#[component]
fn Login() -> impl IntoView {
    view! {
        <div>
            <h2>"Login"</h2>
            <p>"Please log in to access the protected area."</p>
        </div>
    }
}

/// Auth guard — checks authentication status asynchronously
#[component]
fn AuthGuard() -> impl IntoView {
    let auth = LocalResource::new(|| check_auth());

    view! {
        {move || {
            auth.map(|is_authenticated| {
                if *is_authenticated {
                    view! { <Outlet/> }.into_any()
                } else {
                    view! { <Redirect path="/login"/> }.into_any()
                }
            })
        }}
    }
}

/// Protected page — only shown when authenticated
#[component]
fn Dashboard() -> impl IntoView {
    view! {
        <div style="border:2px solid #4CAF50;padding:16px;border-radius:8px;">
            <h2>"Dashboard"</h2>
            <p>"Welcome! You are authenticated and can see this protected content."</p>
        </div>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <Router>
            <nav>
                <A href="/">"Home | "</A>
                <A href="/dashboard">"Dashboard"</A>
            </nav>
            <hr/>
            <Routes fallback=|| "Page not found">
                <Route path=path!("/") view=Home/>
                <Route path=path!("/login") view=Login/>
                <ParentRoute path=path!("/dashboard") view=AuthGuard>
                    <Route path=path!("") view=Dashboard/>
                </ParentRoute>
            </Routes>
        </Router>
    }
}

fn main() {
    mount_to_body(Exercise);
}
