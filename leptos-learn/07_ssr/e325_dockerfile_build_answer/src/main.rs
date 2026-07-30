// ============================================================
// Exercise e325 — dockerfile_build — Answer
//
// Core: Docker multi-stage build, Rust compilation, cargo-leptos build
// ============================================================

use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    let dockerfile = r#"FROM rust:latest AS builder
WORKDIR /app
RUN cargo install cargo-leptos
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release 2>/dev/null || true
COPY . .
RUN cargo leptos build --release

FROM debian:bookworm-slim AS runtime
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release /app
COPY --from=builder /app/site /app/site
EXPOSE 8080
ENV LEPTOS_SITE_ADDR="0.0.0.0:8080"
CMD ["/app/server"]
"#;

    view! {
        <div>
            <h2>"Docker Multi-Stage Build"</h2>
            <p>"Reference Dockerfile for Leptos SSR:"</p>
            <pre style="background:#1e1e1e; color:#d4d4d4; padding:1rem; border-radius:4px; overflow-x:auto; font-size:0.85rem;">{dockerfile}</pre>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
