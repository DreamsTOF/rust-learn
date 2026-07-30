// ============================================================
// Exercise 306 - Answer
// ============================================================
//
// Axum 中间件完整示例 — 请求日志中间件
//
// 注意: 本代码展示 Axum 中间件的标准模式。在真实的 SSR 项目
// 中，此代码位于 src/main.rs 中通过 #[tokio::main] 启动服务器。
// 作为练习答案，以 Leptos 组件形式呈现中间件的完整代码。

use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <div style="padding: 1.5rem; font-family: system-ui, sans-serif; max-width: 48rem; margin: 0 auto;">
            <h1 style="border-bottom: 2px solid #e2e8f0; padding-bottom: 0.5rem;">
                "📡 Axum 中间件 — 参考答案"
            </h1>

            <section style="margin: 1.5rem 0; background: #1e293b; color: #e2e8f0; border-radius: 8px; padding: 1.5rem;">
                <h3 style="color: #38bdf8;">"1️⃣ 请求日志中间件函数"</h3>
                <pre style="font-size: 0.875rem; overflow-x: auto; line-height: 1.6;">
                    {r#"async fn log_middleware<B>(
    request: axum::http::Request<B>,
    next: Next<B>,
) -> impl IntoResponse {
    let start = std::time::Instant::now();
    let method = request.method().clone();
    let uri = request.uri().clone();
    let response = next.run(request).await;
    let duration = start.elapsed();
    println!("[{}] {method} {uri} — {:?}",
        chrono::Local::now().format("%H:%M:%S"),
        duration,
    );
    response
}"#}
                </pre>
            </section>

            <section style="margin: 1.5rem 0; background: #1e293b; color: #e2e8f0; border-radius: 8px; padding: 1.5rem;">
                <h3 style="color: #38bdf8;">"2️⃣ Router 与中间件挂载"</h3>
                <pre style="font-size: 0.875rem; overflow-x: auto; line-height: 1.6;">
                    {r#"use axum::{
    Router, middleware::{self, Next},
    response::IntoResponse,
    routing::{get, post},
};

// 路由处理函数
async fn list_users() -> impl IntoResponse {
    "用户列表"
}

async fn do_login() -> impl IntoResponse {
    "登录成功"
}

let app = Router::new()
    .route("/api/users", get(list_users))
    .route("/api/login", post(do_login))
    // 中间件通过 .layer() 应用到所有路由
    .layer(middleware::from_fn(log_middleware));

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}"#}
                </pre>
            </section>

            <section style="margin: 1.5rem 0; background: #f0f9ff; border-left: 4px solid #38bdf8; border-radius: 8px; padding: 1.5rem;">
                <h3 style="margin-top: 0; color: #0369a1;">"🔑 关键要点"</h3>
                <ul style="line-height: 1.8; color: #334155;">
                    <li>"中间件函数签名为 " <code style="background: #e2e8f0; padding: 0.1rem 0.3rem; border-radius: 2px;">async fn(axum::http::Request&lt;B&gt;, Next&lt;B&gt;) -> impl IntoResponse</code></li>
                    <li>"使用 " <code style="background: #e2e8f0; padding: 0.1rem 0.3rem; border-radius: 2px;">middleware::from_fn(log_middleware)</code>" 将函数转换为中间件层"</li>
                    <li>"通过 " <code style="background: #e2e8f0; padding: 0.1rem 0.3rem; border-radius: 2px;">.layer()</code>" 将中间件应用到 Router"</li>
                    <li>"调 " <code style="background: #e2e8f0; padding: 0.1rem 0.3rem; border-radius: 2px;">next.run(request)</code>" 将请求传递给下一个中间件或路由处理函数"</li>
                </ul>
            </section>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
