// ============================================================
// 练习 e309: Axum 集成与自定义路由
//
// 核心知识点:
//   - 在 Leptos SSR 应用中嵌套自定义 Axum 路由
//   - Router::nest() 路由嵌套
//   - 自定义 API 路由与 Leptos 路由共存
//   - 与 Leptos 共享 Axum Router
//
// 难度: ⭐⭐ (关键 TODO 已标记)
//
// 说明: 本练习展示如何在 Leptos SSR 应用中将自定义 Axum
// 路由与 Leptos 路由共存，实现 API 端点与页面路由的混合架构。
// ============================================================

use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    let (api_routes, _set_api_routes) = signal(vec![
        ("/api/health".to_string(), "GET".to_string(), "健康检查".to_string()),
        ("/api/users".to_string(), "GET".to_string(), "获取用户列表".to_string()),
        ("/api/users".to_string(), "POST".to_string(), "创建用户".to_string()),
        ("/api/data/:id".to_string(), "GET".to_string(), "获取指定数据".to_string()),
    ]);

    view! {
        <div style="padding: 1.5rem; font-family: system-ui, sans-serif; max-width: 48rem; margin: 0 auto;">
            <h1 style="border-bottom: 2px solid #e2e8f0; padding-bottom: 0.5rem;">
                "🔗 Axum 集成与自定义路由"
            </h1>

            <section style="margin: 1.5rem 0;">
                <h2>"路由架构模式"</h2>
                <p style="line-height: 1.6; color: #334155;">
                    "在 Leptos SSR 应用中，Axum Router 作为顶层路由分发器，同时处理 Leptos 页面路由"
                    "和自定义 API 路由。通过 Router::nest() 将 API 路由挂载到指定前缀下。"
                </p>
                <pre style="background: #1e293b; color: #e2e8f0; padding: 1rem; border-radius: 8px; font-size: 0.875rem;">
"请求入口
  │
  ▼
Axum Router (主路由)
  │
  ├── /api/* ─────────► API Router (自定义 REST 端点)
  │     ├── GET    /health
  │     ├── GET    /users
  │     ├── POST   /users
  │     └── GET    /data/:id
  │
  └── /* ───────────► Leptos SSR Router (页面渲染)"
                </pre>
            </section>

            <section style="margin: 1.5rem 0; background: #1e293b; color: #e2e8f0; border-radius: 8px; padding: 1.5rem;">
                <h3 style="color: #38bdf8;">"📝 主 Router 配置"</h3>
                <pre style="font-size: 0.875rem; overflow-x: auto;">
"use axum::{Router, routing::{get, post}};

// ⭐⭐ TODO 1: 创建 API Router（独立的子路由）
// 提示: Router::new()
//          .route(\"/health\", get(health_check))
//          .route(\"/users\", get(list_users))
//          .route(\"/users\", post(create_user))
let api_router = _________________________________________________;
                                ╰──────────────────────────────────╯
// ⭐⭐ TODO 2: 将 API Router 嵌套到主路由的 /api 前缀下
// 提示: Router::new().nest(\"/api\", api_router)
let app = _____________________________;

// ⭐⭐ TODO 3: 添加 Leptos SSR 处理（与自定义路由共存）
// 提示: let app = app.merge(leptos_axum::render_route(...));
// 或使用 .fallback(leptos_axum::render_app_to_stream(...))"
                </pre>
                <p style="color: #fbbf24; margin-top: 1rem;">
                    "⭐ TODO 4: 完成 API 路由处理函数："
                </p>
                <pre style="font-size: 0.875rem; overflow-x: auto;">
"// ⭐⭐ 健康检查端点
async fn health_check() -> impl IntoResponse {
    // 返回 JSON: {\"status\": \"ok\"}
    // 提示: use axum::Json; Json(serde_json::json!({...}))
    Json(serde_json::json!({_______: _______}))
}

// ⭐⭐ 获取用户列表
async fn list_users() -> impl IntoResponse {
    let users = vec![\"Alice\", \"Bob\", \"Charlie\"];
    Json(serde_json::json!(users))
}

// ⭐⭐ 创建用户（接收 JSON body）
async fn create_user(
    // 提示: axum::extract::Json<UserInput>
    Json(payload): Json<serde_json::Value>,
) -> impl IntoResponse {
    (StatusCode::CREATED, Json(payload))
}"
                </pre>
                <p style="color: #94a3b8; font-size: 0.85rem; margin-top: 0.5rem;">
                    "💡 提示: Router::nest(\"/api\", api_router) 将所有 /api/* 请求转发到 api_router。"
                    "使用 .merge() 可以合并两个 Router；路由冲突时后者优先。"
                </p>
            </section>

            <section style="margin: 1.5rem 0; background: #f8fafc; border: 1px solid #e2e8f0; border-radius: 8px; padding: 1.5rem;">
                <h3>"📋 API 路由列表"</h3>
                <table style="width: 100%; border-collapse: collapse;">
                    <thead>
                        <tr style="background: #f1f5f9;">
                            <th style="padding: 0.5rem; text-align: left; border-bottom: 2px solid #e2e8f0;">"路径"</th>
                            <th style="padding: 0.5rem; text-align: left; border-bottom: 2px solid #e2e8f0;">"方法"</th>
                            <th style="padding: 0.5rem; text-align: left; border-bottom: 2px solid #e2e8f0;">"描述"</th>
                        </tr>
                    </thead>
                    <tbody>
                        {move || api_routes.get().into_iter().map(|(path, method, desc)| {
                            let bg = if method == "GET" { "#dbeafe" } else { "#fef3c7" };
                            view! {
                                <tr>
                                    <td style="padding: 0.5rem; border-bottom: 1px solid #e2e8f0; font-family: monospace;">{path}</td>
                                    <td style="padding: 0.5rem; border-bottom: 1px solid #e2e8f0;">
                                        <span style={format!("background: {}; padding: 0.15rem 0.5rem; border-radius: 4px; font-size: 0.8rem", bg)}>
                                            {method}
                                        </span>
                                    </td>
                                    <td style="padding: 0.5rem; border-bottom: 1px solid #e2e8f0;">{desc}</td>
                                </tr>
                            }
                        }).collect::<Vec<_>>()}
                    </tbody>
                </table>
            </section>

            <section style="margin: 1.5rem 0; padding: 1rem; background: #f0f9ff; border-left: 4px solid #38bdf8; border-radius: 4px;">
                <p style="margin: 0; font-size: 0.9rem;">
                    <strong>"🔑 关键概念："</strong>
                    "Router::nest(\"/prefix\", sub_router) 将子路由挂载到指定前缀下；"
                    "Leptos SSR 使用 leptos_axum::render_app_to_stream 生成 HTML 流；"
                    "自定义 API 路由和 Leptos 路由可以无缝共存于同一个 Axum Router。"
                </p>
            </section>

            <details style="margin: 1.5rem 0;">
                <summary style="cursor: pointer; font-weight: 600; color: #2563eb;">
                    "📖 点击展开完整代码参考"
                </summary>
                <pre style="background: #f1f5f9; padding: 1rem; border-radius: 4px; font-size: 0.8rem; overflow-x: auto; margin-top: 0.5rem;">
"use axum::{
    Router, Json, http::StatusCode,
    routing::{get, post},
};
use serde_json::json;

async fn health_check() -> impl IntoResponse {
    Json(json!({ \"status\": \"ok\" }))
}

async fn list_users() -> impl IntoResponse {
    let users = vec![\"Alice\", \"Bob\", \"Charlie\"];
    Json(json!(users))
}

async fn create_user(Json(payload): Json<serde_json::Value>) -> impl IntoResponse {
    (StatusCode::CREATED, Json(payload))
}

#[tokio::main]
async fn main() {
    let api_router = Router::new()
        .route(\"/health\", get(health_check))
        .route(\"/users\", get(list_users))
        .route(\"/users\", post(create_user));

    let app = Router::new()
        .nest(\"/api\", api_router)
        .fallback(leptos_axum::render_app_to_stream(|| view! { <App /> }));

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
