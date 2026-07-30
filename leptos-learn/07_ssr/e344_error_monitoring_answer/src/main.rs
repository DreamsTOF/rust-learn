// ============================================================
// Exercise e344 — Answer: Error Monitoring
//
// Core: Error boundary, panic hook, error reporting, Sentry integration
// ============================================================

use leptos::prelude::*;

const SENTRY_DSN: &str = "https://your-dsn@sentry.io/1234567";

fn init_panic_hook() {
    std::panic::set_hook(Box::new(|panic_info| {
        let payload = panic_info
            .payload()
            .downcast_ref::<String>()
            .map(|s| s.clone())
            .or_else(|| panic_info.payload().downcast_ref::<&str>().map(|s| s.to_string()))
            .unwrap_or_default();

        let location = panic_info
            .location()
            .map(|l| format!("{}:{}", l.file(), l.line()))
            .unwrap_or_default();

        tracing::error!("Panic at {}: {}", location, payload);

        sentry::capture_event(sentry::protocol::Event {
            message: Some(format!("Panic: {}", payload)),
            level: sentry::Level::Fatal,
            ..Default::default()
        });
    }));
}

fn report_error(error_message: &str, file: &str, line: u32) {
    sentry::configure_scope(|scope| {
        scope.set_tag("source", "leptos_app");
        scope.set_extra("file", file.to_string().into());
        scope.set_extra("line", line.into());
    });
    sentry::capture_message(error_message, sentry::Level::Error);
    tracing::error!("[ErrorReport] {} ({}:{})", error_message, file, line);
}

#[component]
fn BuggyComponent(should_error: bool) -> impl IntoView {
    if should_error {
        throw();
    }

    view! {
        <p>"Everything is fine!"</p>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    let (should_error, set_should_error) = signal(false);

    const SENTRY_CODE: &str = "\
use sentry;

fn main() {
    let _guard = sentry::init((\"https://key@sentry.io/project\", sentry::ClientOptions {
        release: sentry::release_name!(),
        environment: Some(\"production\".into()),
        ..Default::default()
    }));
    init_panic_hook();
}

fn init_panic_hook() {
    std::panic::set_hook(Box::new(|panic_info| {
        sentry::capture_event(sentry::protocol::Event {
            message: Some(format!(\"{}\", panic_info)),
            level: sentry::Level::Fatal,
            ..Default::default()
        });
    }));
}";

    view! {
        <div>
            <h1>"Error Monitoring"</h1>

            <section>
                <h2>"Sentry Integration"</h2>
                <pre>{SENTRY_CODE}</pre>
            </section>

            <section>
                <h2>"Error Boundary Demo"</h2>
                <button on:click=move |_| set_should_error.set(true)>
                    "Trigger Error"
                </button>

                <div>
                    <h3>"Outer (Safe Zone)"</h3>
                    <ErrorBoundary fallback=|errors| view! {
                        <div>
                            <p>"Error caught!"</p>
                            <ul>
                                {move || errors.get()
                                    .into_iter()
                                    .map(|e| view! { <li>{e.to_string()}</li> })
                                    .collect::<Vec<_>>()
                                }
                            </ul>
                            <button on:click=move |_| set_should_error.set(false)>
                                "Reset"
                            </button>
                        </div>
                    }>
                        <BuggyComponent should_error=should_error/>
                    </ErrorBoundary>
                </div>
            </section>

            <section>
                <h2>"Error Reporting Flow"</h2>
                <ol>
                    <li>"ErrorBoundary catches child component error"</li>
                    <li>"report_error() formats and tags the error"</li>
                    <li>"sentry::capture_event / capture_message uploads"</li>
                    <li>"Sentry console aggregates, alerts, analyzes"</li>
                    <li>"Developer investigates with stack + context"</li>
                </ol>
            </section>
        </div>
    }
}

fn main() {
    init_panic_hook();
    mount_to_body(Exercise);
}
