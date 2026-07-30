// ============================================================
// 练习 e345: Request Tracing — 请求追踪与结构化日志
//
// 核心知识点:
//   - tracing span: 每个请求一个追踪跨度
//   - JSON 日志: 结构化日志输出
//   - Trace ID 传播: 跨服务传递追踪上下文
//
// 难度: ⭐⭐ (关键 TODOs)
// ============================================================

use leptos::prelude::*;
use leptos::prelude::ServerFnError;
use tracing::info_span;

// TODO: 初始化 tracing-subscriber 为 JSON 格式
// ⭐⭐ 使用 tracing_subscriber::fmt() 配置 JSON 日志输出
// 启用: json(), with_target(true), with_thread_ids(true)
fn init_tracing() {
    // ⭐⭐ TODO: 配置 tracing-subscriber 以 JSON 格式输出
    // 提示:
    // use tracing_subscriber::fmt;
    //
    // fmt()
    //     .json()
    //     .with_target(true)
    //     .with_thread_ids(true)
    //     .with_current_span(true)
    //     .init();
}

// TODO: 生成 Trace ID
// ⭐⭐ 使用 uuid crate 生成 V4 UUID
// 格式化为 32 位十六进制字符串
fn generate_trace_id() -> String {
    // ⭐⭐ TODO: 生成 trace ID
    // 提示: uuid::Uuid::new_v4().to_string()
    "00000000-0000-0000-0000-000000000000".to_string()
}

// TODO: 创建一个带有追踪跨度的 #[server] 函数
// ⭐⭐ 在 server fn 中使用 tracing::info_span! 创建 span
// 注入 trace_id、method、path 等字段

#[server(TracedEndpoint, "/api/traced")]
pub async fn traced_request(method: String, path: String) -> Result<String, ServerFnError> {
    let trace_id = generate_trace_id();

    // ⭐⭐ TODO: 创建并进入追踪 span
    // let span = info_span!(
    //     "request",
    //     trace_id = %trace_id,
    //     http.method = %method,
    //     http.path = %path,
    // );
    // let _guard = span.enter();

    tracing::info!("Processing request");

    // 模拟业务逻辑
    tokio::time::sleep(std::time::Duration::from_millis(50)).await;

    tracing::info!("Request completed");

    Ok(serde_json::json!({
        "trace_id": trace_id,
        "status": "ok",
    }).to_string())
}

// TODO: 实现中间件风格的追踪包装器
// ⭐⭐ 为每个传入请求创建 span，记录开始和结束时间
fn trace_middleware<F, T>(request_name: &str, f: F) -> T
where
    F: FnOnce() -> T,
{
    // ⭐⭐ TODO: 包装函数调用为追踪 span
    // let span = info_span!("middleware", request.name = %request_name);
    // let _guard = span.enter();
    // f()
    f()
}

#[component]
fn Exercise() -> impl IntoView {
    const TRACING_CODE: &str = "\
// tracing-subscriber JSON 配置
use tracing_subscriber::fmt;

fmt()
    .json()
    .with_target(true)
    .with_thread_ids(true)
    .with_current_span(true)
    .init();

// 带 trace_id 的 server fn
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
    // ... 业务逻辑 ...
    tracing::info!(\"Request completed\");

    Ok(json!({\"trace_id\": trace_id}).to_string())
}";

    view! {
        <div>
            <h1>"Request Tracing — 请求追踪"</h1>

            <section>
                <h2>"tracing-subscriber JSON 配置"</h2>
                <pre>{TRACING_CODE}</pre>
            </section>

            <section>
                <h2>"示例 JSON 日志输出"</h2>
                <pre>{"\
{\"timestamp\":\"2026-07-28T12:00:00.123Z\",\"level\":\"INFO\",
 \"fields\":{\"message\":\"Processing request\"},
 \"target\":\"leptos_app\",
 \"spans\":[{\"request\":{\"trace_id\":\"a1b2c3d4-...\",
   \"http.method\":\"GET\",\"http.path\":\"/api/data\"}}],
 \"thread_id\":42}"}</pre>
            </section>

            <section>
                <h2>"Trace ID 传播"</h2>
                <table>
                    <thead>
                        <tr><th>"层级"</th><th>"Trace ID 载体"</th><th>"说明"</th></tr>
                    </thead>
                    <tbody>
                        <tr><td>"HTTP 请求"</td><td>"X-Trace-ID 请求头"</td><td>"客户端生成或上游传入"</td></tr>
                        <tr><td>"服务端 span"</td><td>"tracing span"</td><td>"每个请求一个 span，包含 trace_id 字段"</td></tr>
                        <tr><td>"日志输出"</td><td>"JSON 字段"</td><td>"结构化日志包含 span 上下文"</td></tr>
                        <tr><td>"下游调用"</td><td>"HTTP 头 / gRPC metadata"</td><td>"传播 trace_id 到子服务"</td></tr>
                    </tbody>
                </table>
            </section>

            <section>
                <h2>"配置关键点"</h2>
                <ul>
                    <li>"使用 .json() 启用结构化输出"</li>
                    <li>"with_current_span(true) 自动附加当前 span"</li>
                    <li>"info_span! 创建带字段的 span"</li>
                    <li>"span.enter() 激活 span（_guard 析构时退出）"</li>
                </ul>
            </section>
        </div>
    }
}

fn main() {
    init_tracing();
    mount_to_body(Exercise);
}
