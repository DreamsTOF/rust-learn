// ============================================================
// Exercise 223 - Answer (pending_guard)
// ============================================================

use leptos::prelude::*;
use leptos_router::components::{Router, Routes, Route, ParentRoute, A, Redirect};
use leptos_router::components::Outlet;
use leptos_router::path;

/// Simulate auth check
async fn slow_auth_check() -> bool {
    true
}

/// Home page
#[component]
fn Home() -> impl IntoView {
    view! {
        <div>
            <h2>"Home"</h2>
            <p>"Public homepage — always visible."</p>
        </div>
    }
}

/// Pending guard — shows a loading indicator while auth is checked
#[component]
fn PendingGuard() -> impl IntoView {
    let auth = LocalResource::new(|| slow_auth_check());

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
       // Show a loading state while the resource is pending
       {move || {
            if auth.get().is_none() {
               view! {
                   <div style="text-align:center;padding:40px;">
                       <div style="display:inline-block;width:40px;height:40px;border:4px solid #f3f3f3;border-top:4px solid #3498db;border-radius:50%;animation:spin 1s linear infinite;"></div>
                        <p>"Verifying authentication..."</p>
                    </div>
                }.into_any()
            } else {
                view! {}.into_any()
            }
        }}
    }
}

/// Protected settings page
#[component]
fn Settings() -> impl IntoView {
    view! {
        <div style="border:2px solid #2196F3;padding:16px;border-radius:8px;">
            <h2>"Settings"</h2>
            <p>"This is a protected page. You can only see it after passing the guard."</p>
        </div>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <Router>
            <nav>
                <A href="/">"Home | "</A>
                <A href="/settings">"Settings"</A>
            </nav>
            <hr/>
            <Routes fallback=|| "Page not found">
                <Route path=path!("/") view=Home/>
                <ParentRoute path=path!("/settings") view=PendingGuard>
                    <Route path=path!("") view=Settings/>
                </ParentRoute>
            </Routes>
        </Router>
    }
}

fn main() {
    mount_to_body(Exercise);
}
