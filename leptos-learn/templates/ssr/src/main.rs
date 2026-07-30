// ============================================================
// 练习 {NUMBER}: {TITLE}
//
// 目标: {DESCRIPTION}
//
// 难度: {DIFFICULTY}
// 核心知识点: {CONCEPTS}
// ============================================================

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let routes = generate_route_list(App);

    let app = Router::new()
        .leptos_routes(&conf, routes, App)
        .fallback(leptos_axum::file_and_error_handler)
        .with_state(conf);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    leptos::log!("Listening on http://{}", addr);
    axum::serve(listener, app).await.unwrap();
}

#[cfg(not(feature = "ssr"))]
fn main() {
    // 客户端入口 - hydrate
}

#[component]
fn App() -> impl IntoView {
    view! {
        <div>
            <h1>"{TITLE}"</h1>
            // TODO: 实现练习内容
        </div>
    }
}
