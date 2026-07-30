// ============================================================
// Exercise 224 - Answer (route_transition)
// ============================================================

use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::path;

#[component]
fn Home() -> impl IntoView {
    view! {
        <h2>"Home"</h2>
        <p>"This content transitions smoothly during route changes."</p>
    }
}

#[component]
fn About() -> impl IntoView {
    view! {
        <h2>"About"</h2>
        <p>"Route transitions make navigation feel seamless."</p>
    }
}

#[component]
fn Contact() -> impl IntoView {
    view! {
        <h2>"Contact"</h2>
        <p>"Contact us at transition@example.com"</p>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    let (is_routing, set_is_routing) = signal(false);
    // Derive CSS class from routing state:
    // - During routing → "route-container exiting" (old content fades out)
    // - After routing  → "route-container" (new content fades in via CSS transition)
    let container_class = Signal::derive(move || {
        if is_routing() { "route-container exiting" } else { "route-container" }
    });

    view! {
        <Router set_is_routing=set_is_routing>
            <h1>"e224: Route Transition"</h1>
            <nav style="margin-bottom:16px;">
                <span style="margin-right:12px;"><A href="/">"Home"</A></span>
                <span style="margin-right:12px;"><A href="/about">"About"</A></span>
                <A href="/contact">"Contact"</A>
            </nav>
            <main>
                <div class=container_class>
                    <Routes fallback=|| "Page not found">
                        <Route path=path!("/") view=Home/>
                        <Route path=path!("/about") view=About/>
                        <Route path=path!("/contact") view=Contact/>
                    </Routes>
                </div>
            </main>
        </Router>
    }
}

fn main() {
    mount_to_body(Exercise);
}
