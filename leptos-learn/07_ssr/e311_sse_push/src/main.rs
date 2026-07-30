// ============================================================
// 练习 e311: SSE 服务端推送 (Server-Sent Events)
//
// 核心知识点:
//   - SSE 协议：服务器向客户端单向推送事件流
//   - EventSource：浏览器端的 SSE 接收 API
//   - Axum 中的 SSE 端点实现
//   - 事件流（Stream）的构建与发送
//
// 难度: ⭐⭐⭐ (探索式练习 — 最少提示)
//
// 说明: SSE 允许服务器通过 HTTP 连接持续向客户端推送数据。
// 与 WebSocket 不同，SSE 是单向的（服务器→客户端），
// 基于纯 HTTP，天然支持重连和事件 ID。
// 本练习要求你根据已有的概念提示，自行补全代码。
// ============================================================

use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    let (events, _set_events) = signal(vec![
        "2024-01-15 10:00:01 [info] 服务启动完成".to_string(),
        "2024-01-15 10:00:05 [warning] 连接池初始化".to_string(),
        "2024-01-15 10:00:12 [info] 用户 alice 登录".to_string(),
        "2024-01-15 10:00:30 [error] 数据库查询超时".to_string(),
        "2024-01-15 10:01:00 [info] 自动重连成功".to_string(),
    ]);

    view! {
        <div style="padding: 1.5rem; font-family: system-ui, sans-serif; max-width: 48rem; margin: 0 auto;">
            <h1 style="border-bottom: 2px solid #e2e8f0; padding-bottom: 0.5rem;">
                "📡 SSE 服务端推送 (Server-Sent Events)"
            </h1>

            <section style="margin: 1.5rem 0;">
                <h2>"SSE 工作原理"</h2>
                <p style="line-height: 1.6; color: #334155;">
                    "SSE (Server-Sent Events) 是一种轻量级的服务器推送技术。客户端通过"
                    "EventSource API 建立 HTTP 长连接，服务器持续发送事件流数据。"
                </p>
                <div style="background: #1e293b; color: #e2e8f0; padding: 1rem; border-radius: 8px; font-size: 0.875rem; margin: 1rem 0; font-family: monospace;">
                    "客户端                             服务器\n  │                                  │\n  │───── GET /events （普通 HTTP）─────►│\n  │                                  │\n  │◄──── text/event-stream ──────────│\n  │◄──── data: {\"msg\": \"hello\"}  ─────│\n  │◄──── data: {\"count\": 1}     ─────│\n  │◄──── event: update\ndata: ... ────│\n  │           ...                       │"
                </div>
            </section>

            <section style="margin: 1.5rem 0; background: #1e293b; color: #e2e8f0; border-radius: 8px; padding: 1.5rem;">
                <h3 style="color: #38bdf8;">"🖥️ Axum SSE 服务端实现"</h3>
                <pre style="font-size: 0.875rem; overflow-x: auto;">
"use axum::response::sse::{Event, Sse};
use futures::stream::Stream;
use tokio_stream::wrappers::ReceiverStream;

// SSE 端点：返回事件流
async fn sse_handler() -> Sse<impl Stream<Item = Result<Event, ???>>> {
    let (tx, rx) = tokio::sync::mpsc::channel::<String>(32);

    // 后台任务持续发送事件
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(1));
        loop {
            interval.tick().await;
            let data = format!(\"data: {{ \\\"time\\\": \\\"{}\\\" }}\n\n\", chrono::Local::now());
            if tx.send(data).await.is_err() { break; }
        }
    });

    // 将 Receiver 转换为 SSE 事件流
    let stream = ReceiverStream::new(rx).map(|msg| {
        Ok(Event::default().data(msg))
    });

    Sse::new(stream)
        .keep_alive(KeepAlive::new()
            .interval(Duration::from_secs(15))
            .text(\"keep-alive\"))
}"
                </pre>
                <p style="color: #fbbf24; margin-top: 1rem;">
                    "💡 关键类型与方法："
                </p>
                <ul style="color: #94a3b8; font-size: 0.9rem;">
                    <li>"Sse::new(stream) — 创建 SSE 响应"</li>
                    <li>"Event::default().data(msg) — 构造事件"</li>
                    <li>"Event::default().event(\"name\").data(msg) — 命名事件"</li>
                    <li>".keep_alive(...) — 保持连接活跃"</li>
                    <li>"use axum::response::sse::{Event, Sse, KeepAlive};"</li>
                    <li>"use tokio_stream::wrappers::ReceiverStream;"</li>
                </ul>
            </section>

            <section style="margin: 1.5rem 0; background: #1e293b; color: #e2e8f0; border-radius: 8px; padding: 1.5rem;">
                <h3 style="color: #38bdf8;">"🌐 客户端 EventSource"</h3>
                <pre style="font-size: 0.875rem; overflow-x: auto;">
"// 浏览器端 JavaScript
const eventSource = new EventSource('/api/events');

// 监听消息事件
eventSource.onmessage = (event) => {
    console.log('收到:', event.data);
};

// 监听命名事件
eventSource.addEventListener('update', (event) => {
    console.log('update 事件:', event.data);
});

// 错误处理（自动重连）
eventSource.onerror = (err) => {
    console.error('连接错误，浏览器将自动重连...');
};

// 关闭连接
// eventSource.close();"
                </pre>
            </section>

            <section style="margin: 1.5rem 0; background: #f8fafc; border: 1px solid #e2e8f0; border-radius: 8px; padding: 1.5rem;">
                <h3>"📋 模拟事件流"</h3>
                <div style="background: #0f172a; color: #22c55e; padding: 1rem; border-radius: 8px; font-family: monospace; font-size: 0.8rem; max-height: 200px; overflow-y: auto;">
                    {move || events.get().into_iter().map(|event| {
                        view! { <div>{event}</div> }
                    }).collect::<Vec<_>>()}
                </div>
            </section>

            <section style="margin: 1.5rem 0; padding: 1rem; background: #fef2f2; border-left: 4px solid #ef4444; border-radius: 4px;">
                <p style="margin: 0; font-size: 0.9rem;">
                    <strong>"⚠️ SSE vs WebSocket 对比："</strong>
                    "SSE 是单向（服务器→客户端），基于纯 HTTP，自动重连；"
                    "WebSocket 是双向，需要 ws:// 协议，需要额外库。"
                    "SSE 适合实时通知、日志流、状态更新等场景。"
                </p>
            </section>

            <details style="margin: 1.5rem 0;">
                <summary style="cursor: pointer; font-weight: 600; color: #2563eb;">
                    "📖 点击展开完整代码参考"
                </summary>
                <pre style="background: #f1f5f9; padding: 1rem; border-radius: 4px; font-size: 0.8rem; overflow-x: auto; margin-top: 0.5rem;">
"use std::time::Duration;
use axum::{
    Router, response::sse::{Event, Sse, KeepAlive},
    routing::get,
};
use futures::stream::{Stream, StreamExt};
use tokio_stream::wrappers::ReceiverStream;

async fn sse_handler() -> Sse<impl Stream<Item = Result<Event, std::convert::Infallible>>> {
    let (tx, rx) = tokio::sync::mpsc::channel::<String>(32);

    tokio::spawn(async move {
        let mut counter = 0u64;
        loop {
            tokio::time::sleep(Duration::from_secs(2)).await;
            counter += 1;
            let data = format!(\"data: {{ \\\"count\\\": {counter} }}\n\n\");
            if tx.send(data).await.is_err() { break; }
        }
    });

    let stream = ReceiverStream::new(rx).map(|msg| Ok(Event::default().data(msg)));
    Sse::new(stream).keep_alive(KeepAlive::new()
        .interval(Duration::from_secs(15))
        .text(\"keep-alive\"))
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route(\"/api/events\", get(sse_handler));
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
