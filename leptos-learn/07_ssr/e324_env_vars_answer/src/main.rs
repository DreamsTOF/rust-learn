// ============================================================
// Exercise e324 — env_vars — Answer
//
// Core: compile-time (env!) and runtime (std::env::var) environment variables
// ============================================================

use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    // Compile-time — embedded into the binary at build time
    let pkg_version = env!("CARGO_PKG_VERSION");
    let build_date = option_env!("BUILD_DATE").unwrap_or("not set at build time");
    let pkg_name = env!("CARGO_PKG_NAME");

    // Runtime — resolved when the server process runs (SSR/server only)
    // On WASM (client) this would fail, but this is an SSR exercise.
    let runtime_var = match std::env::var("HOSTNAME") {
        Ok(val) => val,
        Err(_) => "N/A (runtime env var not set)".to_string(),
    };

    view! {
        <div>
            <h2>"Environment Variables"</h2>
            <table>
                <tr>
                    <td><strong>"Package Name (compile-time):"</strong></td>
                    <td>{pkg_name}</td>
                </tr>
                <tr>
                    <td><strong>"Version (compile-time):"</strong></td>
                    <td>{pkg_version}</td>
                </tr>
                <tr>
                    <td><strong>"Build Date (compile-time):"</strong></td>
                    <td>{build_date}</td>
                </tr>
                <tr>
                    <td><strong>"HOSTNAME (runtime):"</strong></td>
                    <td>{runtime_var}</td>
                </tr>
            </table>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
