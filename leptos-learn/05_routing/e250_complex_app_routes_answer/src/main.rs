// ============================================================
// Exercise 250 - Answer: Complex App Routes
// ============================================================

use std::sync::Arc;
use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::hooks::*;
use leptos_router::NavigateOptions;
use leptos_router::path;

// ============================================================
// Authentication context
// ============================================================

#[derive(Clone, Copy)]
struct AuthContext(RwSignal<bool>);

impl AuthContext {
    fn new() -> Self {
        Self(RwSignal::new(false))
    }

    fn is_authenticated(&self) -> bool {
        self.0.get()
    }

    fn login(&self) {
        self.0.set(true);
    }

    fn logout(&self) {
        self.0.set(false);
    }
}

// ============================================================
// Auth guard component
// ============================================================

#[component]
fn RequireAuth(children: ChildrenFn) -> impl IntoView {
    let auth = use_context::<AuthContext>().expect("AuthContext not found");
    let navigate = use_navigate();

    Effect::new(move || {
        if !auth.is_authenticated() {
            navigate("/auth/login", NavigateOptions::default());
        }
    });

    view! {
        {move || {
            if auth.is_authenticated() {
                children()
            } else {
                ().into_any()
            }
        }}
    }
}

// ============================================================
// Login page
// ============================================================

#[component]
fn LoginPage() -> impl IntoView {
    let auth = use_context::<AuthContext>().expect("AuthContext not found");
    let navigate = use_navigate();

    let on_login = move |_| {
        auth.login();
        navigate("/dashboard", NavigateOptions::default());
    };

    view! {
        <div class="auth-page">
            <h2>"Login"</h2>
            <p>"Click the button to simulate login."</p>
            <input type="text" placeholder="Username (any)" />
            <input type="password" placeholder="Password (any)" />
            <button class="btn btn-primary" on:click=on_login>
                "Sign In"
            </button>
        </div>
    }
}

// ============================================================
// Dashboard page
// ============================================================

#[component]
fn DashboardPage() -> impl IntoView {
    view! {
        <>
            <h2>"Dashboard"</h2>
            <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 1rem;">
                <div class="card">
                    <h3>"Users"</h3>
                    <p style="font-size: 2rem; font-weight: bold; color: #4361ee;">"1,234"</p>
                </div>
                <div class="card">
                    <h3>"Revenue"</h3>
                    <p style="font-size: 2rem; font-weight: bold; color: #2ec4b6;">"$12.5K"</p>
                </div>
                <div class="card">
                    <h3>"Orders"</h3>
                    <p style="font-size: 2rem; font-weight: bold; color: #e71d36;">"567"</p>
                </div>
                <div class="card">
                    <h3>"Growth"</h3>
                    <p style="font-size: 2rem; font-weight: bold; color: #ff9f1c;">"+18%"</p>
                </div>
            </div>
        </>
    }
}

// ============================================================
// Settings pages (with nested sub-routes)
// ============================================================

#[component]
fn SettingsLayout() -> impl IntoView {
    view! {
        <>
            <h2>"Settings"</h2>
            <nav style="display: flex; gap: 0.5rem; margin-bottom: 1rem; border-bottom: 2px solid #eee; padding-bottom: 0.5rem;">
                <A href="/settings" exact=true>"Profile"</A>
                <A href="/settings/account">"Account"</A>
            </nav>
            <Outlet />
        </>
    }
}

#[component]
fn ProfileSettings() -> impl IntoView {
    view! {
        <div class="card">
            <h3>"Profile Settings"</h3>
            <div>
                <label>"Display Name"</label>
                <input type="text" value="John Doe" style="display: block; width: 100%; padding: 0.5rem; margin: 0.25rem 0 1rem; box-sizing: border-box;" />
            </div>
            <div>
                <label>"Email"</label>
                <input type="email" value="john@example.com" style="display: block; width: 100%; padding: 0.5rem; margin: 0.25rem 0 1rem; box-sizing: border-box;" />
            </div>
            <div>
                <label>"Bio"</label>
                <textarea style="display: block; width: 100%; padding: 0.5rem; margin: 0.25rem 0; box-sizing: border-box; height: 80px;">"Rust developer"</textarea>
            </div>
        </div>
    }
}

#[component]
fn AccountSettings() -> impl IntoView {
    view! {
        <div class="card">
            <h3>"Account Settings"</h3>
            <div style="margin-bottom: 1rem;">
                <label>"Password"</label>
                <input type="password" value="********" style="display: block; width: 100%; padding: 0.5rem; margin: 0.25rem 0; box-sizing: border-box;" />
            </div>
            <div style="margin-bottom: 1rem;">
                <label>"Two-Factor Auth"</label>
                <div><input type="checkbox" checked /> " Enabled"</div>
            </div>
            <div>
                <label>"Delete Account"</label>
                <div><button class="btn btn-outline" style="color: #e71d36; border-color: #e71d36;">"Delete"</button></div>
            </div>
        </div>
    }
}

// ============================================================
// Profile page
// ============================================================

#[component]
fn ProfilePage() -> impl IntoView {
    view! {
        <>
            <h2>"Profile"</h2>
            <div class="card" style="display: flex; gap: 1.5rem; align-items: center;">
                <div style="width: 80px; height: 80px; border-radius: 50%; background: #4361ee; color: #fff; display: flex; align-items: center; justify-content: center; font-size: 2rem; font-weight: bold;">
                    "J"
                </div>
                <div>
                    <h3 style="margin: 0 0 0.25rem;">"John Doe"</h3>
                    <p style="margin: 0; color: #666;">"Senior Rust Developer"</p>
                    <p style="margin: 0.25rem 0 0; color: #666;">"john@example.com"</p>
                </div>
            </div>
            <div class="card">
                <h3>"Recent Activity"</h3>
                <ul>
                    <li>"Updated profile picture — 2 days ago"</li>
                    <li>"Changed password — 1 week ago"</li>
                    <li>"Enabled two-factor auth — 2 weeks ago"</li>
                </ul>
            </div>
        </>
    }
}

// ============================================================
// App layout (sidebar + content area)
// ============================================================

#[component]
fn AppLayout() -> impl IntoView {
    let auth = use_context::<AuthContext>().expect("AuthContext not found");
    let navigate = Arc::new(use_navigate());

    view! {
        <RequireAuth>
            <div class="layout">
                <nav class="sidebar">
                    <h3>"My App"</h3>
                    <A href="/dashboard">"📊 Dashboard"</A>
                    <A href="/settings">"⚙️ Settings"</A>
                    <A href="/profile">"👤 Profile"</A>
                    <hr style="border-color: #2a2a4a; margin: 1rem 0;" />
                    <button
                        class="btn btn-outline"
                        style="width: 100%; color: #e71d36; border-color: #e71d36; background: transparent; padding: 0.5rem; cursor: pointer;"
                        on:click={
                            let nav = Arc::clone(&navigate);
                            move |_| {
                                auth.logout();
                                nav("/auth/login", NavigateOptions { replace: true, ..Default::default() });
                            }
                        }
                    >
                        "🚪 Logout"
                    </button>
                </nav>
                <main class="content">
                    <Outlet />
                </main>
            </div>
        </RequireAuth>
    }
}

// ============================================================
// Redirect component & entry point
// ============================================================

#[component]
fn RedirectTo(path: &'static str) -> impl IntoView {
    let navigate = use_navigate();
    Effect::new(move || {
        navigate(path, NavigateOptions { replace: true, ..Default::default() });
    });
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        let auth = AuthContext::new();
        provide_context(auth);

        view! {
            <Router>
                <Routes fallback=|| "Page not found.">
                    <Route path=path!("auth/login") view=LoginPage />
                    <ParentRoute path=path!("/") view=AppLayout>
                        <Route path=path!("dashboard") view=DashboardPage />
                        <ParentRoute path=path!("settings") view=SettingsLayout>
                            <Route path=path!("") view=ProfileSettings />
                            <Route path=path!("account") view=AccountSettings />
                        </ParentRoute>
                        <Route path=path!("profile") view=ProfilePage />
                        <Route path=path!("") view=|| view! { <RedirectTo path="/dashboard" /> } />
                    </ParentRoute>
                </Routes>
            </Router>
        }
    });
}
