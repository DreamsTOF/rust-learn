// ============================================================
// Exercise 225 - Answer (guard_compose)
// ============================================================

use leptos::prelude::*;
use leptos_router::components::{Router, Routes, Route, ParentRoute, A, Redirect};
use leptos_router::components::Outlet;
use leptos_router::path;

/// Simulate checking auth status
async fn check_auth() -> bool {
    true
}

/// Simulate fetching user role
async fn fetch_role() -> String {
    "editor".to_string()
}

/// Home page
#[component]
fn Home() -> impl IntoView {
    view! {
        <div>
            <h2>"Home"</h2>
            <p>"Welcome to the app!"</p>
        </div>
    }
}

/// Forbidden page
#[component]
fn Forbidden() -> impl IntoView {
    view! {
        <div>
            <h2>"403 Forbidden"</h2>
            <p>"You don't have permission to access this page."</p>
        </div>
    }
}

/// Layer 1: Auth guard — checks if user is logged in
#[component]
fn AuthGuard() -> impl IntoView {
    let auth = LocalResource::new(|| check_auth());

    view! {
        {move || {
            auth.map(|is_authenticated| {
                if *is_authenticated {
                    view! { <Outlet/> }.into_any()
                } else {
                    view! { <Redirect path="/"/> }.into_any()
                }
            })
        }}
    }
}

/// Layer 2: Role guard — checks if user has the required role
#[component]
fn RoleGuard(required_role: String) -> impl IntoView {
    let role = LocalResource::new(|| fetch_role());

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

/// Protected editor page — requires auth + editor role
#[component]
fn EditorPanel() -> impl IntoView {
    view! {
        <div style="border:2px solid #4CAF50;padding:16px;border-radius:8px;">
            <h2>"Editor Panel"</h2>
            <p>"You are authenticated and have the required role to access this page."</p>
        </div>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <Router>
            <nav>
                <A href="/">"Home | "</A>
                <A href="/editor">"Editor Panel"</A>
            </nav>
            <hr/>
            <Routes fallback=|| "Page not found">
                <Route path=path!("/") view=Home/>
                <Route path=path!("/forbidden") view=Forbidden/>
                <ParentRoute path=path!("/editor") view=AuthGuard>
                    <ParentRoute path=path!("") view=move || view! { <RoleGuard required_role="editor".to_string()/> }>
                        <Route path=path!("") view=EditorPanel/>
                    </ParentRoute>
                </ParentRoute>
            </Routes>
        </Router>
    }
}

fn main() {
    mount_to_body(Exercise);
}
