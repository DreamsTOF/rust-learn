// ============================================================
// 练习 e310: Axum 共享状态 (Shared State)
//
// 核心知识点:
//   - AppState 模式：在路由间共享应用状态
//   - State 提取器：从请求中获取状态
//   - Extension 扩展机制
//   - 共享数据库连接池与配置信息
//
// 难度: ⭐⭐ (关键 TODO 已标记)
//
// 说明: 本练习展示如何在 Axum 路由处理函数之间共享状态，
// 例如数据库连接池、配置信息和缓存等。使用 State 提取器
// 在路由处理函数中获取共享状态。
// ============================================================

use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    let (state_fields, _set_state_fields) = signal(vec![
        ("pool".to_string(), "PgPool (数据库连接池)".to_string(), "Database".to_string()),
        ("config".to_string(), "AppConfig (应用配置)".to_string(), "Config".to_string()),
        ("cache".to_string(), "Cache (数据缓存)".to_string(), "Cache".to_string()),
        ("metrics".to_string(), "MetricsRegistry (指标)".to_string(), "Telemetry".to_string()),
    ]);

    view! {
        <div style="padding: 1.5rem; font-family: system-ui, sans-serif; max-width: 48rem; margin: 0 auto;">
            <h1 style="border-bottom: 2px solid #e2e8f0; padding-bottom: 0.5rem;">
                "📦 Axum 共享状态 (Shared State)"
            </h1>

            <section style="margin: 1.5rem 0;">
                <h2>"为什么需要共享状态？"</h2>
                <p style="line-height: 1.6; color: #334155;">
                    "在 Web 应用中，多个路由处理函数通常需要访问相同的资源——数据库连接池、"
                    "配置信息、缓存等。Axum 提供了 State 提取器和 Extension 机制来共享状态。"
                </p>
                <ul style="line-height: 1.8; color: #334155;">
                    <li>"🎯 <strong>State 提取器</strong>：类型安全、编译期检查的状态访问方式"</li>
                    <li>"🔌 <strong>Extension</strong>：基于请求扩展的轻量级状态传递"</li>
                    <li>"📋 <strong>AppState</strong>：将多个资源合并为一个结构体"</li>
                </ul>
            </section>

            <section style="margin: 1.5rem 0; background: #1e293b; color: #e2e8f0; border-radius: 8px; padding: 1.5rem;">
                <h3 style="color: #38bdf8;">"📝 AppState 结构体定义"</h3>
                <pre style="font-size: 0.875rem; overflow-x: auto;">
"use std::sync::Arc;
use sqlx::PgPool;

// ⭐⭐ TODO 1: 定义 AppState 结构体
// 提示: 包含 db_pool (PgPool) 和 config (AppConfig) 字段
#[derive(Clone)]
struct AppState {
    // ⭐⭐ 数据库连接池
    pub db_pool: _________,
    // ⭐⭐ 应用配置
    pub config: Arc<_________>,
}

// ⭐⭐ TODO 2: 创建应用状态实例
// 提示: let state = AppState { db_pool, config: Arc::new(config) };
let state = _________________________________;"
                </pre>
                <p style="color: #fbbf24; margin-top: 1rem;">
                    "⭐ TODO 3: 将状态注入到 Router 中："
                </p>
                <pre style="font-size: 0.875rem; overflow-x: auto;">
"let app = Router::new()
    .route(\"/api/users\", get(list_users))
    .route(\"/api/users/:id\", get(get_user))
    .route(\"/api/config\", get(get_config))
    // ⭐⭐ 使用 .with_state() 注入共享状态
    // 提示: .with_state(state)
    .________________;"
                </pre>
                <p style="color: #94a3b8; font-size: 0.85rem; margin-top: 0.5rem;">
                    "💡 提示: 使用 .with_state(state) 注入状态。该状态必须实现 Clone。"
                </p>
            </section>

            <section style="margin: 1.5rem 0; background: #1e293b; color: #e2e8f0; border-radius: 8px; padding: 1.5rem;">
                <h3 style="color: #38bdf8;">"🔧 使用 State 提取器"</h3>
                <pre style="font-size: 0.875rem; overflow-x: auto;">
"use axum::extract::State;

// ⭐⭐ TODO 4: 使用 State 提取器获取 AppState
async fn list_users(
    // 提示: State(state): State<AppState>
    ___________________,
) -> impl IntoResponse {
    // 从 state 中获取数据库连接
    let pool = &state.db_pool;
    // 执行数据库查询...
    Json(json!([\"Alice\", \"Bob\", \"Charlie\"]))
}

// ⭐⭐ TODO 5: 在路由处理函数中同时使用 State 和路径参数
async fn get_user(
    // 提示: State(state): State<AppState>, Path(id): Path<i32>
    ____________________________________________,
) -> impl IntoResponse {
    // 从 config 中读取相关设置
    let max_users = state.config.max_users;
    Json(json!({\"id\": id, \"max_users\": max_users}))
}

// ⭐⭐ TODO 6: Extension 方式（替代 State）
async fn get_config(
    Extension(config): Extension<Arc<AppConfig>>,
) -> impl IntoResponse {
    Json(json!(config))
}"
                </pre>
            </section>

            <section style="margin: 1.5rem 0; background: #f8fafc; border: 1px solid #e2e8f0; border-radius: 8px; padding: 1.5rem;">
                <h3>"📋 共享状态字段"</h3>
                <table style="width: 100%; border-collapse: collapse;">
                    <thead>
                        <tr style="background: #f1f5f9;">
                            <th style="padding: 0.5rem; text-align: left; border-bottom: 2px solid #e2e8f0;">"字段名"</th>
                            <th style="padding: 0.5rem; text-align: left; border-bottom: 2px solid #e2e8f0;">"类型"</th>
                            <th style="padding: 0.5rem; text-align: left; border-bottom: 2px solid #e2e8f0;">"用途"</th>
                        </tr>
                    </thead>
                    <tbody>
                        {move || state_fields.get().into_iter().map(|(field, ftype, usage)| {
                            view! {
                                <tr>
                                    <td style="padding: 0.5rem; border-bottom: 1px solid #e2e8f0; font-family: monospace;">{field}</td>
                                    <td style="padding: 0.5rem; border-bottom: 1px solid #e2e8f0;">{ftype}</td>
                                    <td style="padding: 0.5rem; border-bottom: 1px solid #e2e8f0;">{usage}</td>
                                </tr>
                            }
                        }).collect::<Vec<_>>()}
                    </tbody>
                </table>
            </section>

            <section style="margin: 1.5rem 0; padding: 1rem; background: #f0f9ff; border-left: 4px solid #38bdf8; border-radius: 4px;">
                <p style="margin: 0; font-size: 0.9rem;">
                    <strong>"🔑 关键概念："</strong>
                    "State<T> 提取器需要 Router 通过 .with_state() 注入了 T 类型的状态；"
                    "状态必须实现 Clone（通常使用 Arc 包装）；"
                    "也可通过 Extension<T> 在中间件中传递状态。"
                </p>
            </section>

            <details style="margin: 1.5rem 0;">
                <summary style="cursor: pointer; font-weight: 600; color: #2563eb;">
                    "📖 点击展开完整代码参考"
                </summary>
                <pre style="background: #f1f5f9; padding: 1rem; border-radius: 4px; font-size: 0.8rem; overflow-x: auto; margin-top: 0.5rem;">
"use std::sync::Arc;
use axum::{Router, extract::{State, Path}, response::IntoResponse,
            routing::{get}, Extension};
use serde_json::json;

#[derive(Clone)]
struct AppConfig {
    max_users: usize,
    app_name: String,
}

#[derive(Clone)]
struct AppState {
    pub config: Arc<AppConfig>,
}

async fn list_users(State(state): State<AppState>) -> impl IntoResponse {
    Json(json!([\"Alice\", \"Bob\", \"Charlie\"]))
}

async fn get_user(State(state): State<AppState>, Path(id): Path<i32>) -> impl IntoResponse {
    Json(json!({\"id\": id, \"max_users\": state.config.max_users}))
}

async fn get_config(Extension(config): Extension<Arc<AppConfig>>) -> impl IntoResponse {
    Json(json!(config))
}

#[tokio::main]
async fn main() {
    let config = Arc::new(AppConfig { max_users: 100, app_name: \"MyApp\".into() });
    let state = AppState { config: config.clone() };

    let app = Router::new()
        .route(\"/api/users\", get(list_users))
        .route(\"/api/users/:id\", get(get_user))
        .route(\"/api/config\", get(get_config))
        .layer(Extension(config))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(\"0.0.0.0:3000\").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
"
                </pre>
            </details>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
