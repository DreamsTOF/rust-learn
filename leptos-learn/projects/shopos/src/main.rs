#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use shopos::app::App;
    use shopos::db::seed::run_seed;
    use sqlx::sqlite::SqlitePoolOptions;
    use tower_http::cors::CorsLayer;
    use tower_sessions::Expiry;
    use tower_sessions::cookie::Key;
    use tower_sessions::session_store::SessionStore;
    use tracing_subscriber::EnvFilter;

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .init();

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite:shopos.db?mode=rwc")
        .await
        .expect("Failed to create pool");

    sqlx::migrate!("migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    run_seed(&pool).await;

    let session_store = tower_sessions::MemoryStore::default();
    let session_key = Key::generate();
    let session_layer = tower_sessions::SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_expiry(Expiry::OnInactivity(std::time::Duration::from_secs(86400)))
        .with_signed(session_key);

    let conf = get_configuration(None).unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    let app = Router::new()
        .leptos_routes(&leptos_options, routes, App)
        .fallback(leptos_axum::file_and_error_handler)
        .layer(session_layer)
        .layer(CorsLayer::permissive())
        .with_state(leptos_options);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    leptos::logging::log!("ShopOS listening on http://{}", addr);
    axum::serve(listener, app).await.unwrap();
}

#[cfg(not(feature = "ssr"))]
fn main() {
    use leptos::prelude::*;
    _ = console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(shopos::app::App);
}
