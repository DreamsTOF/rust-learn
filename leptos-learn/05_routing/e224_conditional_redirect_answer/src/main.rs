// ============================================================
// Exercise 224 - Answer (conditional_redirect)
// ============================================================

use leptos::prelude::*;
use leptos_router::components::{Router, Routes, Route, ParentRoute, A, Redirect};
use leptos_router::components::Outlet;
use leptos_router::path;

/// Simulate checking if user has completed onboarding
async fn check_onboarding_status() -> bool {
    true // Simulate: onboarding already completed
}

/// Dashboard — the main app page
#[component]
fn Dashboard() -> impl IntoView {
    view! {
        <div>
            <h2>"Dashboard"</h2>
            <p>"Welcome to your dashboard!"</p>
        </div>
    }
}

/// Onboarding page — shown if user hasn't completed onboarding
#[component]
fn Onboarding() -> impl IntoView {
    view! {
        <div style="border:2px solid #9C27B0;padding:16px;border-radius:8px;">
            <h2>"Onboarding"</h2>
            <p>"Please complete the onboarding process to continue."</p>
        </div>
    }
}

/// Conditional guard — redirects based on application state
/// If onboarding is not complete, redirect to /onboarding
#[component]
fn OnboardingGuard() -> impl IntoView {
    let onboarding_complete = LocalResource::new(|| check_onboarding_status());

    view! {
        {move || {
            onboarding_complete.map(|completed| {
                if *completed {
                    view! { <Outlet/> }.into_any()
                } else {
                    view! { <Redirect path="/onboarding"/> }.into_any()
                }
            })
        }}
    }
}

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <Router>
            <nav>
                <A href="/">"Dashboard | "</A>
                <A href="/onboarding">"Onboarding"</A>
            </nav>
            <hr/>
            <Routes fallback=|| "Page not found">
                <Route path=path!("/onboarding") view=Onboarding/>
                <ParentRoute path=path!("/") view=OnboardingGuard>
                    <Route path=path!("") view=Dashboard/>
                </ParentRoute>
            </Routes>
        </Router>
    }
}

fn main() {
    mount_to_body(Exercise);
}
