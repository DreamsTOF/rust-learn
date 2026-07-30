// ============================================================
// Exercise e345 — Answer: Request Tracing
//
// Core: tracing span per request, JSON logs, trace ID propagation
// ============================================================

use leptos::prelude::*;
use leptos::prelude::ServerFnError;
use tracing::info_span;

fn init_tracing() {
    use tracing_subscriber::fmt;
    fmt()
        .json()
        .with_target(true)
        .with_thread_ids(true)
        .with_current_span(true)
        .init();
}

fn generate_trace_id() -> String {
    uuid::Uuid::new_v4().to_string()
}

#[server(TracedEndpoint, "/api/traced")]
pub async fn traced_request(method: String, path: String) -> Result<String, ServerFnError> {
    let trace_id = generate_trace_id();

    let span = info_span!(
        "request",
        trace_id = %trace_id,
        http.method = %method,
        http.path = %path,
    );
    let _guard = span.enter();

    tracing::info!("Processing request");

    tokio::time::sleep(std::time::Duration::from_millis(50)).await;

    tracing::info!("Request completed");

    Ok(serde_json::json!({
        "trace_id": trace_id,
        "status": "ok",
    }).to_string())
}

fn trace_middleware<F, T>(request_name: &str, f: F) -> T
where
    F: FnOnce() -> T,
{
    let span = info_span!("middleware", request.name = %request_name);
    let _guard = span.enter();
    f()
}

#[component]
fn Exercise() -> impl IntoView {
    const TRACING_CODE: &str = "\
use tracing_subscriber::fmt;
use tracing::info_span;

// JSON logging setup
fmt().json()
    .with_target(true)
    .with_thread_ids(true)
    .with_current_span(true)
    .init();

// Traced server function
#[server(TracedEndpoint, \"/api/traced\")]
pub async fn traced_request(
    method: String,
    path: String,
) -> Result<String, ServerFnError> {
    let trace_id = uuid::Uuid::new_v4().to_string();
    let span = info_span!(
        \"request\",
        trace_id = %trace_id,
        http.method = %method,
        http.path = %path,
    );
    let _guard = span.enter();
    tracing::info!(\"Processing request\");
    // ... business logic ...
    tracing::info!(\"Request completed\");
    Ok(json!({\"trace_id\": trace_id, \"status\": \"ok\"}).to_string())
}";

    view! {
        <div>
            <h1>"Request Tracing"</h1>

            <section>
                <h2>"tracing-subscriber JSON Setup"</h2>
                <pre>{TRACING_CODE}</pre>
            </section>

            <section>
                <h2>"Sample JSON Log Output"</h2>
                <pre>{"\
{\"timestamp\":\"2026-07-28T12:00:00.123Z\",\"level\":\"INFO\",
 \"fields\":{\"message\":\"Processing request\"},
 \"target\":\"leptos_app\",
 \"spans\":[{\"request\":{\"trace_id\":\"a1b2c3d4-...\",
   \"http.method\":\"GET\",\"http.path\":\"/api/data\"}}],
 \"thread_id\":42}"}</pre>
            </section>

            <section>
                <h2>"Trace ID Propagation"</h2>
                <table>
                    <thead>
                        <tr><th>"Layer"</th><th>"Trace ID Carrier"</th><th>"Description"</th></tr>
                    </thead>
                    <tbody>
                        <tr><td>"HTTP Request"</td><td>"X-Trace-ID header"</td><td>"Client-generated or upstream"</td></tr>
                        <tr><td>"Server span"</td><td>"tracing span"</td><td>"Per-request span with trace_id field"</td></tr>
                        <tr><td>"Log output"</td><td>"JSON field"</td><td>"Structured log carries span context"</td></tr>
                        <tr><td>"Downstream call"</td><td>"HTTP header / gRPC metadata"</td><td>"Propagate trace_id to child services"</td></tr>
                    </tbody>
                </table>
            </section>

            <section>
                <h2>"Key Configuration Points"</h2>
                <ul>
                    <li>".json() enables structured output"</li>
                    <li>"with_current_span(true) auto-attaches current span"</li>
                    <li>"info_span! creates spans with fields"</li>
                    <li>"span.enter() activates the span (dropped when _guard goes out of scope)"</li>
                </ul>
            </section>
        </div>
    }
}

fn main() {
    init_tracing();
    mount_to_body(Exercise);
}
