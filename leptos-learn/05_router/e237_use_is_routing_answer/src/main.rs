// ============================================================
// Exercise 237 - Answer
// ============================================================

use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::path;

#[component]
fn Exercise() -> impl IntoView {
    let (is_routing, set_is_routing) = signal(false);
    let routing_text = move || {
        if is_routing() {
            "路由切换中..."
        } else {
            "路由空闲"
        }
    };

    view! {
        <Router set_is_routing=set_is_routing>
            <nav>
                <A href="/">"Home"</A>
                " | "
                <A href="/about">"About"</A>
            </nav>
            <main>
                <h3>"路由状态: " {routing_text}</h3>
                <Routes fallback=|| "Not found">
                    <Route path=path!("/") view=|| view! { <p>"Home"</p> }/>
                    <Route path=path!("/about") view=|| view! { <p>"About page"</p> }/>
                </Routes>
            </main>
        </Router>
    }
}

fn main() {
    mount_to_body(Exercise);
}
