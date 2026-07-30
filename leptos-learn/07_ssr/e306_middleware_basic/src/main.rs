// ============================================================
// 练习 e306: Axum 中间件基础 (Middleware Basics)
//
// 核心知识点:
//   - Axum middleware 概念与作用（请求/响应拦截）
//   - tower::Service 与 Layer trait
//   - 使用 axum::middleware::from_fn 添加自定义中间件
//   - 请求日志记录中间件
//
// 难度: ⭐⭐ (关键 TODO 已标记)
//
// 说明: 本练习通过展示 Axum 中间件的代码示例，让你理解
// 中间件的工作原理。你将看到如何编写一个简单的请求日志
// 中间件，并将其应用到路由上。
// ============================================================

use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    let (example_routes, _set_example_routes) = signal(vec![
        "GET  /api/users".to_string(),
        "POST /api/login".to_string(),
        "GET  /api/items".to_string(),
    ]);

    // 代码示例字符串（在 view! 外定义为 raw string，避免内嵌引号和花括号的解析冲突）
    let todo1_code = r#"async fn log_middleware<B>(
    // ⭐⭐ TODO 1: 填写 request 参数类型
    // 提示: axum::http::Request<B>
    request: _________,
    next: Next<B>,
) -> impl IntoResponse {
    let start = std::time::Instant::now();
    let method = request.method().clone();
    let uri = request.uri().clone();
    let response = next.run(request).await;
    let duration = start.elapsed();
    println!("[{}] {} {} — {:?}",
        chrono::Local::now().format("%H:%M:%S"),
        method, uri, duration
    );
    response
}"#;
    let todo2_code = r#"let app = Router::new()
    .route("/api/users", get(list_users))
    .route("/api/login", post(do_login))
    // ⭐⭐ 在这里添加中间件层:
    // .___________________________________;"#;
    let code_ref = r#"use axum::{
    Router, middleware::{self, Next},
    response::IntoResponse,
    routing::get,
};

async fn log_middleware<B>(
    request: axum::http::Request<B>,
    next: Next<B>,
) -> impl IntoResponse {
    let start = std::time::Instant::now();
    let method = request.method().clone();
    let uri = request.uri().clone();
    let response = next.run(request).await;
    println!("[{}] {method} {uri} — {:?}", start.elapsed());
    response
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/api/users", get(list_users))
        .route("/api/login", post(do_login))
        .layer(middleware::from_fn(log_middleware));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}"#;

    view! {
        <div style="padding: 1.5rem; font-family: system-ui, sans-serif; max-width: 48rem; margin: 0 auto;">
            <h1 style="border-bottom: 2px solid #e2e8f0; padding-bottom: 0.5rem;">
                "📡 Axum 中间件基础"
            </h1>

            <section style="margin: 1.5rem 0;">
                <h2>"什么是中间件？"</h2>
                <p style="line-height: 1.6; color: #334155;">
                    "中间件是位于客户端请求与路由处理函数之间的处理层。它可以："
                </p>
                <ul style="line-height: 1.8; color: #334155;">
                    <li>"✅ 记录请求日志（方法、路径、耗时）"</li>
                    <li>"✅ 验证身份与权限"</li>
                    <li>"✅ 添加响应头"</li>
                    <li>"✅ 请求限流与压缩"</li>
                </ul>
            </section>

            <section style="margin: 1.5rem 0; background: #1e293b; color: #e2e8f0; border-radius: 8px; padding: 1.5rem;">
                <h3 style="color: #38bdf8;">"📝 中间件函数示例（日志记录）"</h3>
                <pre style="font-size: 0.875rem; overflow-x: auto;">{todo1_code}</pre>
                <p style="color: #fbbf24; margin-top: 1rem;">
                    "⭐ TODO 2: 将中间件应用到路由："
                </p>
                <pre style="font-size: 0.875rem; overflow-x: auto;">{todo2_code}</pre>
                <p style="color: #94a3b8; font-size: 0.85rem; margin-top: 0.5rem;">
                    "💡 提示: 使用 .layer(axum::middleware::from_fn(log_middleware)) 可以将中间件应用到所有路由上。"
                </p>
            </section>

            <section style="margin: 1.5rem 0; background: #f8fafc; border: 1px solid #e2e8f0; border-radius: 8px; padding: 1.5rem;">
                <h3>"📋 模拟请求日志输出"</h3>
                <ul>
                    {move || example_routes.get().into_iter().map(|route| {
                        view! { <li style="font-family: monospace; color: #334155;">{route}</li> }
                    }).collect::<Vec<_>>()}
                </ul>
            </section>

            <section style="margin: 1.5rem 0; padding: 1rem; background: #f0f9ff; border-left: 4px solid #38bdf8; border-radius: 4px;">
                <p style="margin: 0; font-size: 0.9rem;">
                    <strong>"🔑 关键概念："</strong>
                    "Next<B> 代表下一个中间件或路由处理函数；"
                    "next.run(request) 将请求传递给下游；"
                    "from_fn() 将普通函数转换为中间件层。"
                </p>
            </section>

            <details style="margin: 1.5rem 0;">
                <summary style="cursor: pointer; font-weight: 600; color: #2563eb;">
                    "📖 点击展开完整代码参考"
                </summary>
                <pre style="background: #f1f5f9; padding: 1rem; border-radius: 4px; font-size: 0.8rem; overflow-x: auto; margin-top: 0.5rem;">{code_ref}</pre>
            </details>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
