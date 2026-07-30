// ============================================================
// Exercise e343 — Answer: Health Check
//
// Core: /health /_ready /_live endpoints, dependency health
// ============================================================

use leptos::prelude::*;
use leptos::prelude::ServerFnError;

#[derive(Debug, Clone, PartialEq)]
enum HealthStatus {
    Healthy,
    Degraded,
    Unhealthy,
}

#[derive(Debug, Clone)]
struct DependencyHealth {
    name: String,
    status: HealthStatus,
    latency_ms: u64,
    error: Option<String>,
}

#[derive(Debug, Clone)]
struct HealthCheckResponse {
    overall_status: HealthStatus,
    version: String,
    uptime_seconds: u64,
    dependencies: Vec<DependencyHealth>,
    timestamp: String,
}

async fn check_database_health() -> DependencyHealth {
    let start = std::time::Instant::now();
    // In production: sqlx::PgPool or SqlitePool ping
    let status = HealthStatus::Healthy;
    let latency_ms = start.elapsed().as_millis() as u64;
    DependencyHealth {
        name: "PostgreSQL".to_string(),
        status,
        latency_ms,
        error: None,
    }
}

async fn check_cache_health() -> DependencyHealth {
    let start = std::time::Instant::now();
    // In production: redis::Client PING
    let status = HealthStatus::Healthy;
    let latency_ms = start.elapsed().as_millis() as u64;
    DependencyHealth {
        name: "Redis".to_string(),
        status,
        latency_ms,
        error: None,
    }
}

async fn check_external_api_health() -> DependencyHealth {
    let start = std::time::Instant::now();
    // In production: reqwest::Client head request
    let status = HealthStatus::Healthy;
    let latency_ms = start.elapsed().as_millis() as u64;
    DependencyHealth {
        name: "External API".to_string(),
        status,
        latency_ms,
        error: None,
    }
}

async fn aggregate_health() -> HealthCheckResponse {
    let (db, cache, api) = tokio::join!(
        check_database_health(),
        check_cache_health(),
        check_external_api_health(),
    );

    let dependencies = vec![db, cache, api];

    let overall_status = if dependencies.iter().all(|d| d.status == HealthStatus::Healthy) {
        HealthStatus::Healthy
    } else if dependencies.iter().any(|d| d.status == HealthStatus::Unhealthy) {
        HealthStatus::Unhealthy
    } else {
        HealthStatus::Degraded
    };

    HealthCheckResponse {
        overall_status,
        version: env!("CARGO_PKG_VERSION").to_string(),
        uptime_seconds: 0,
        dependencies,
        timestamp: chrono::Utc::now().to_rfc3339(),
    }
}

#[server(HealthEndpoint, "/api/health")]
pub async fn health_check() -> Result<String, ServerFnError> {
    let report = aggregate_health().await;
    Ok(serde_json::to_string(&report).unwrap_or_default())
}

#[server(ReadyEndpoint, "/api/_ready")]
pub async fn ready_check() -> Result<String, ServerFnError> {
    let db = check_database_health().await;
    if db.status == HealthStatus::Healthy {
        Ok("OK".to_string())
    } else {
        Err(ServerFnError::new("Database not ready"))
    }
}

#[server(LiveEndpoint, "/api/_live")]
pub async fn live_check() -> Result<String, ServerFnError> {
    Ok("OK".to_string())
}

#[component]
fn Exercise() -> impl IntoView {
    const HEALTH_CODE: &str = "\
#[server(HealthEndpoint, \"/api/health\")]
pub async fn health_check() -> Result<String, ServerFnError> {
    let report = aggregate_health().await;
    Ok(serde_json::to_string(&report)?)
}

#[server(ReadyEndpoint, \"/api/_ready\")]
pub async fn ready_check() -> Result<String, ServerFnError> {
    let db = check_database_health().await;
    if db.status == HealthStatus::Healthy { Ok(\"OK\") }
    else { Err(ServerFnError::new(\"DB not ready\")) }
}

#[server(LiveEndpoint, \"/api/_live\")]
pub async fn live_check() -> Result<String, ServerFnError> {
    Ok(\"OK\")
}";

    view! {
        <div>
            <h1>"Health Check Endpoints"</h1>

            <section>
                <h2>"Endpoint Design"</h2>
                <table>
                    <thead>
                        <tr><th>"Endpoint"</th><th>"Purpose"</th><th>"Response"</th></tr>
                    </thead>
                    <tbody>
                        <tr><td><code>"/health"</code></td><td>"Aggregate health"</td><td>"JSON: overall status + dependency details"</td></tr>
                        <tr><td><code>"/_ready"</code></td><td>"Readiness (K8s)"</td><td>"Core dependency check"</td></tr>
                        <tr><td><code>"/_live"</code></td><td>"Liveness (K8s)"</td><td>"Simple OK (process alive)"</td></tr>
                    </tbody>
                </table>
            </section>

            <section>
                <h2>"Server Functions"</h2>
                <pre>{HEALTH_CODE}</pre>
            </section>

            <section>
                <h2>"Sample Response"</h2>
                <pre>{"\
{
  \"overall_status\": \"healthy\",
  \"version\": \"0.1.0\",
  \"uptime_seconds\": 86400,
  \"dependencies\": [
    { \"name\": \"PostgreSQL\", \"status\": \"healthy\", \"latency_ms\": 5 },
    { \"name\": \"Redis\",       \"status\": \"healthy\", \"latency_ms\": 2 },
    { \"name\": \"External API\",\"status\": \"healthy\", \"latency_ms\": 120 }
  ],
  \"timestamp\": \"2026-07-28T12:00:00Z\"
}"}</pre>
            </section>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
