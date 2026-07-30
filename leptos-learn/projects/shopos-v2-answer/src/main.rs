#![recursion_limit = "256"]

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use shopos_v2_answer::app::App;
    use shopos_v2_answer::db::seed::run_seed;
    use sqlx::sqlite::SqlitePoolOptions;
    use tower_http::cors::CorsLayer;
    use tower_sessions::Expiry;

    tracing_subscriber::fmt()
        .init();

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite:shopos.db?mode=rwc")
        .await
        .expect("Failed to create pool");

    // Run migrations
    let migration_sql = include_str!("../migrations/001_initial.sql");
    sqlx::raw_sql(migration_sql).execute(&pool).await.expect("Migration failed");

    run_seed(&pool).await;

    let session_store = tower_sessions::MemoryStore::default();
    let session_layer = tower_sessions::SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_expiry(Expiry::OnInactivity(time::Duration::seconds(86400)));

    let conf = get_configuration(None).unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    let app = Router::new()
        .leptos_routes(&leptos_options, routes, App)
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
    leptos::mount::hydrate_body(shopos_v2_answer::app::App);
}
