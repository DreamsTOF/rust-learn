// ============================================================
// Exercise 238 - Answer
// ============================================================

use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::path;

// Lazy-loaded page component (in a real app, this would be in a separate
// module for code splitting, loaded on demand when the route is accessed)
fn LazyPage() -> impl IntoView {
    view! { <p>"This page was loaded lazily"</p> }
}

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <Router>
            <nav>
                <A href="/">"Home"</A>
                <A href="/lazy">"Lazy Page"</A>
            </nav>
            <main>
                <Routes fallback=|| "Not found">
                    <Route path=path!("/") view=|| view! { <p>"Home Page"</p> }/>
                    <Route path=path!("/lazy") view=LazyPage/>
                </Routes>
            </main>
        </Router>
    }
}

fn main() {
    mount_to_body(Exercise);
}
