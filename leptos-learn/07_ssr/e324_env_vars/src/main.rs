use leptos::prelude::*;

// TODO: Show compile-time and runtime environment variables
//
// Core: env!("VAR") for compile-time (must be set at build), std::env::var("VAR") for runtime
//
// Hints:
//   1. Define #[component] fn Exercise() -> impl IntoView
//   2. Use env!("CARGO_PKG_VERSION") for compile-time version
//   3. Use option_env!("BUILD_DATE") for optional compile-time var
//   4. For runtime env vars in SSR: std::env::var("HOSTNAME") — only works server-side
//   5. On client (WASM), std::env::var is unavailable — show an alternative

#[component]
fn Exercise() -> impl IntoView {
    // Compile-time env vars — embedded into binary
    let pkg_version = env!("CARGO_PKG_VERSION");
    let build_date = option_env!("BUILD_DATE").unwrap_or("not set at build time");
    let pkg_name = env!("CARGO_PKG_NAME");

    // Runtime env vars — only work in SSR/server context
    // TODO: Try std::env::var("HOSTNAME") or another runtime variable
    // let runtime_var = match std::env::var("HOSTNAME") {
    //     Ok(val) => val,
    //     Err(_) => "N/A (runtime env var not set)".to_string(),
    // };

    view! {
        <div>
            <h2>"Environment Variables"</h2>
            <p><strong>"Package Name (compile-time):"</strong> {pkg_name}</p>
            <p><strong>"Version (compile-time):"</strong> {pkg_version}</p>
            <p><strong>"Build Date (compile-time):"</strong> {build_date}</p>
            // TODO: Display runtime env var value here
            <p>"练习 324 (env_vars)"</p>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
