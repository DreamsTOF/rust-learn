use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::path;

fn main() {
    mount_to_body(Exercise);
}

// TODO: 创建一个懒加载的页面组件（模拟按需加载）
// 在实际应用中，此组件会放在单独的文件中实现代码分割

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
                </Routes>
            </main>
        </Router>
    }
}
