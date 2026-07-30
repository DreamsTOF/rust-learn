// ============================================================
// Exercise 222 - Answer (role_guard)
// ============================================================

use leptos::prelude::*;
use leptos_router::components::{Router, Routes, Route, ParentRoute, A, Redirect};
use leptos_router::components::Outlet;
use leptos_router::path;

/// Simulate fetching current user role
async fn fetch_user_role() -> String {
    "admin".to_string() // Simulate an admin user
}

/// Home page — public
#[component]
fn Home() -> impl IntoView {
    view! {
        <div>
            <h2>"Home"</h2>
            <p>"Welcome! This page is accessible to everyone."</p>
        </div>
    }
}

/// Forbidden page
#[component]
fn Forbidden() -> impl IntoView {
    view! {
        <div style="border:2px solid #f44336;padding:16px;border-radius:8px;">
            <h2>"403 Forbidden"</h2>
            <p>"You do not have the required role to access this page."</p>
        </div>
    }
}

/// Role guard — checks user role asynchronously
/// Only users with the required role can see the children
#[component]
fn RoleGuard(required_role: String) -> impl IntoView {
    let role = LocalResource::new(|| fetch_user_role());

    view! {
        {move || {
           role.map(|user_role| {
                if user_role.as_str() == required_role {
                   view! { <Outlet/> }.into_any()
               } else {
                   view! { <Redirect path="/forbidden"/> }.into_any()
                }
            })
        }}
    }
}

/// Admin panel — only accessible to admin role
#[component]
fn AdminPanel() -> impl IntoView {
    view! {
        <div style="border:2px solid #ff9800;padding:16px;border-radius:8px;">
            <h2>"Admin Panel"</h2>
            <p>"This content is only visible to users with the 'admin' role."</p>
        </div>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <Router>
            <nav>
                <A href="/">"Home | "</A>
                <A href="/admin">"Admin Panel"</A>
            </nav>
            <hr/>
            <Routes fallback=|| "Page not found">
                <Route path=path!("/") view=Home/>
                <Route path=path!("/forbidden") view=Forbidden/>
                <ParentRoute path=path!("/admin") view=move || view! { <RoleGuard required_role="admin".to_string()/> }>
                    <Route path=path!("") view=AdminPanel/>
                </ParentRoute>
            </Routes>
        </Router>
    }
}

fn main() {
    mount_to_body(Exercise);
}
