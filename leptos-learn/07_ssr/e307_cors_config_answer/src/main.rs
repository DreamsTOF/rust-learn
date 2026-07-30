// ============================================================
// 练习 e307: CORS 配置 — 参考答案
//
// 核心知识点:
//   - CORS 概念与跨域请求流程
//   - tower_http::cors::CorsLayer 配置
//   - 允许的 Origin / Method / Header 设置
//   - 预检请求 (Preflight / OPTIONS) 处理
// ============================================================

use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    let (allowed_origins, _set_allowed_origins) = signal(vec![
        "http://localhost:3000".to_string(),
        "http://localhost:8080".to_string(),
        "https://myapp.example.com".to_string(),
    ]);

    let (allowed_methods, _set_allowed_methods) = signal(vec![
        "GET".to_string(),
        "POST".to_string(),
        "PUT".to_string(),
        "DELETE".to_string(),
        "OPTIONS".to_string(),
    ]);

    view! {
        <div style="padding: 1.5rem; font-family: system-ui, sans-serif; max-width: 48rem; margin: 0 auto;">
            <h1 style="border-bottom: 2px solid #e2e8f0; padding-bottom: 0.5rem;">
                "🔒 CORS 配置 (Cross-Origin Resource Sharing)"
            </h1>

            <section style="margin: 1.5rem 0;">
                <h2>"什么是 CORS？"</h2>
                <p style="line-height: 1.6; color: #334155;">
                    "CORS（跨域资源共享）是一种浏览器安全机制，用于控制不同源之间的资源访问。"
                    "当浏览器发起跨域请求时，服务器必须返回正确的 CORS 头部，否则请求会被阻止。"
                </p>
                <ul style="line-height: 1.8; color: #334155;">
                    <li>"🌐 简单请求：GET/HEAD/POST，Content-Type 有限制"</li>
                    <li>"⚠️ 预检请求：使用 OPTIONS 方法先询问服务器是否允许实际请求"</li>
                    <li>"🔑 关键头部：Access-Control-Allow-Origin、Access-Control-Allow-Methods、Access-Control-Allow-Headers"</li>
                </ul>
            </section>

            <section style="margin: 1.5rem 0; background: #1e293b; color: #e2e8f0; border-radius: 8px; padding: 1.5rem;">
                <h3 style="color: #38bdf8;">"📝 Router CORS 配置示例"</h3>
                <pre style="font-size: 0.875rem; overflow-x: auto;">
"use tower_http::cors::{CorsLayer, Any};

// 方式一：允许所有来源（开发环境）
let app = Router::new()
    .route(\"/api/data\", get(get_data))
    .route(\"/api/submit\", post(submit_data))
    .layer(CorsLayer::permissive());

// 方式二：受限配置（生产环境）
let cors = CorsLayer::new()
    .allow_origin([
        \"http://localhost:3000\".parse().unwrap(),
        \"https://myapp.example.com\".parse().unwrap(),
    ])
    .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
    .allow_headers([\"content-type\".parse().unwrap(), \"authorization\".parse().unwrap()])
    .allow_credentials(true)
    .max_age(std::time::Duration::from_secs(86400));"
                </pre>
            </section>

            <section style="margin: 1.5rem 0; background: #1e293b; color: #e2e8f0; border-radius: 8px; padding: 1.5rem;">
                <h3 style="color: #38bdf8;">"🔄 预检请求 (Preflight) 流程"</h3>
                <pre style="font-size: 0.875rem; overflow-x: auto;">
"// 浏览器自动发送 OPTIONS 请求，包含以下头部：
// Access-Control-Request-Method: PUT
// Access-Control-Request-Headers: content-type, authorization
// Origin: http://localhost:3000
//
// 服务器应返回：
// Access-Control-Allow-Origin: http://localhost:3000
// Access-Control-Allow-Methods: GET, POST, PUT, DELETE
// Access-Control-Allow-Headers: content-type, authorization
// Access-Control-Max-Age: 86400

// 处理 OPTIONS 预检请求：
// CorsLayer 自动处理 OPTIONS 请求，无需手动添加路由"
                </pre>
            </section>

            <section style="margin: 1.5rem 0; background: #f8fafc; border: 1px solid #e2e8f0; border-radius: 8px; padding: 1.5rem;">
                <h3>"📋 已配置的 CORS 策略概览"</h3>
                <p style="color: #475569; font-size: 0.9rem;">
                    <strong>"允许的来源 (Allowed Origins)："</strong>
                </p>
                <ul>
                    {move || allowed_origins.get().into_iter().map(|origin| {
                        view! { <li style="font-family: monospace; color: #2563eb;">{origin}</li> }
                    }).collect::<Vec<_>>()}
                </ul>
                <p style="color: #475569; font-size: 0.9rem; margin-top: 1rem;">
                    <strong>"允许的方法 (Allowed Methods)："</strong>
                </p>
                <ul>
                    {move || allowed_methods.get().into_iter().map(|method| {
                        view! { <li style="font-family: monospace; color: #059669;">{method}</li> }
                    }).collect::<Vec<_>>()}
                </ul>
            </section>

            <section style="margin: 1.5rem 0; padding: 1rem; background: #f0f9ff; border-left: 4px solid #38bdf8; border-radius: 4px;">
                <p style="margin: 0; font-size: 0.9rem;">
                    <strong>"🔑 关键概念："</strong>
                    "CorsLayer::permissive() 允许所有跨域请求（开发环境适用）；"
                    "生产环境应限制具体 Origin/Method/Header；"
                    "allow_credentials(true) 不能与 allow_origin(Any) 同时使用。"
                </p>
            </section>

            <details style="margin: 1.5rem 0;">
                <summary style="cursor: pointer; font-weight: 600; color: #2563eb;">
                    "📖 点击展开完整代码参考"
                </summary>
                <pre style="background: #f1f5f9; padding: 1rem; border-radius: 4px; font-size: 0.8rem; overflow-x: auto; margin-top: 0.5rem;">
"use axum::{Router, http::Method, routing::{get, post}};
use tower_http::cors::{CorsLayer, Any};

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_origin([
            \"http://localhost:3000\".parse().unwrap(),
            \"https://myapp.example.com\".parse().unwrap(),
        ])
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers([\"content-type\".parse().unwrap(), \"authorization\".parse().unwrap()])
        .allow_credentials(true)
        .max_age(std::time::Duration::from_secs(86400));

    let app = Router::new()
        .route(\"/api/data\", get(get_data))
        .route(\"/api/submit\", post(submit_data))
        .layer(cors);

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
