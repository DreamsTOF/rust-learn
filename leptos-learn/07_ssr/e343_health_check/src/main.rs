// ============================================================
// 练习 e343: Health Check — 服务健康检查端点
//
// 核心知识点:
//   - /health: 聚合健康状态
//   - /_ready: 就绪检查
//   - /_live: 存活检查
//   - 依赖健康: 数据库、缓存、外部服务
//
// 难度: ⭐⭐ (关键 TODOs)
// ============================================================

use leptos::prelude::*;
use leptos::prelude::ServerFnError;

/// 健康检查状态
#[derive(Debug, Clone, PartialEq)]
enum HealthStatus {
    Healthy,
    Degraded,
    Unhealthy,
}

/// 依赖项健康信息
#[derive(Debug, Clone)]
struct DependencyHealth {
    name: String,
    status: HealthStatus,
    latency_ms: u64,
    error: Option<String>,
}

// TODO: 定义完整健康检查响应结构体
// ⭐⭐ 提示: 包含 overall_status, version, uptime_seconds,
// dependencies: Vec<DependencyHealth>, timestamp
#[derive(Debug, Clone)]
struct HealthCheckResponse {
    overall_status: HealthStatus,
    version: String,
    uptime_seconds: u64,
    dependencies: Vec<DependencyHealth>,
    timestamp: String,
}

// TODO: 实现数据库健康检查
// ⭐⭐ 连接数据库并执行 SELECT 1，返回 DependencyHealth
// 包含连接延迟 latency_ms
async fn check_database_health() -> DependencyHealth {
    // ⭐⭐ TODO: 实现数据库 ping
    // 提示: 使用 sqlx::PgPool::acquire() 或 sqlx::SqlitePool
    // 计时连接耗时作为 latency_ms
    // 失败时 status = Unhealthy, error = Some(message)
    DependencyHealth {
        name: "PostgreSQL".to_string(),
        status: HealthStatus::Healthy,
        latency_ms: 5,
        error: None,
    }
}

// TODO: 实现 Redis 缓存健康检查
// ⭐⭐ 使用 PING 命令检查连接
async fn check_cache_health() -> DependencyHealth {
    DependencyHealth {
        name: "Redis".to_string(),
        status: HealthStatus::Healthy,
        latency_ms: 2,
        error: None,
    }
}

// TODO: 实现外部 API 健康检查
// ⭐⭐ 发送 HTTP HEAD 请求到外部健康端点
async fn check_external_api_health() -> DependencyHealth {
    DependencyHealth {
        name: "External API".to_string(),
        status: HealthStatus::Healthy,
        latency_ms: 120,
        error: None,
    }
}

// 聚合健康检查
async fn aggregate_health() -> HealthCheckResponse {
    // TODO: 并行检查所有依赖
    // ⭐⭐ 使用 tokio::join! 或 futures::join_all
    let (db, cache, api) = tokio::join!(
        check_database_health(),
        check_cache_health(),
        check_external_api_health(),
    );

    let dependencies = vec![db, cache, api];

    // 计算整体状态
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
        uptime_seconds: 0, // TODO: 跟踪启动时间
        dependencies,
        timestamp: chrono::Utc::now().to_rfc3339(),
    }
}

// TODO: 实现 #[server] 健康检查端点
// ⭐⭐ 为 /health, /_ready, /_live 分别定义 server fn
// /_live: 仅检查进程是否存活（返回简单 OK）
// /_ready: 检查依赖是否就绪
// /health: 完整的聚合健康检查

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
// 健康检查端点定义

#[server(HealthEndpoint, \"/api/health\")]
pub async fn health_check() -> Result<String, ServerFnError> {
    let report = aggregate_health().await;
    Ok(serde_json::to_string(&report)?)
}

#[server(ReadyEndpoint, \"/api/_ready\")]
pub async fn ready_check() -> Result<String, ServerFnError> {
    let db = check_database_health().await;
    if db.status == HealthStatus::Healthy {
        Ok(\"OK\")
    } else {
        Err(ServerFnError::new(\"Database not ready\"))
    }
}

#[server(LiveEndpoint, \"/api/_live\")]
pub async fn live_check() -> Result<String, ServerFnError> {
    Ok(\"OK\")
}";

    view! {
        <div>
            <h1>"Health Check — 服务健康检查"</h1>

            <section>
                <h2>"端点设计"</h2>
                <table>
                    <thead>
                        <tr>
                            <th>"端点"</th>
                            <th>"用途"</th>
                            <th>"返回"</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td><code>"/health"</code></td>
                            <td>"聚合健康状态"</td>
                            <td>"JSON: 整体状态 + 各依赖详情"</td>
                        </tr>
                        <tr>
                            <td><code>"/_ready"</code></td>
                            <td>"就绪检查 (K8s)"</td>
                            <td>"仅检查数据库等核心依赖"</td>
                        </tr>
                        <tr>
                            <td><code>"/_live"</code></td>
                            <td>"存活检查 (K8s)"</td>
                            <td>"简单返回 OK (进程存活)"</td>
                        </tr>
                    </tbody>
                </table>
            </section>

            <section>
                <h2>"Server Function 定义"</h2>
                <pre>{HEALTH_CODE}</pre>
            </section>

            <section>
                <h2>"响应示例"</h2>
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
