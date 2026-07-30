// ============================================================
// 练习 197: retry_logic — 自动重试(指数退避)
//
// 目标: 实现失败自动重试，每次重试延迟指数增长
//
// 难度: ⭐⭐⭐
// 核心知识点: 自动重试、指数退避
//
// TODO:
//   1. 实现 sleep + 伪随机工具函数
//   2. unreliable_request: 60% 概率失败
//   3. fetch_with_retry: 最多重试 N 次, 每次延迟 2^(attempt-1)*200ms
//   4. 按钮触发请求, 显示尝试次数和结果
// ============================================================

use std::sync::atomic::{AtomicU32, Ordering};
use std::time::Duration;
use futures::channel::oneshot;
use leptos::prelude::*;
use leptos::task::spawn_local;

async fn sleep(ms: u64) {
    let (tx, rx) = oneshot::channel::<()>();
    set_timeout(move || { let _ = tx.send(()); }, Duration::from_millis(ms));
    let _ = rx.await;
}

/// 简单伪随机, 无需 js-sys
fn pseudo_random() -> f64 {
    static COUNTER: AtomicU32 = AtomicU32::new(1);
    let n = COUNTER.fetch_add(1, Ordering::Relaxed);
    let n = n.wrapping_mul(1103515245).wrapping_add(12345);
    (n >> 16) as f64 / 65536.0
}

/// 不可靠请求: 60% 概率失败
async fn unreliable_request() -> Result<String, String> {
    sleep(300).await;
    if pseudo_random() > 0.4 {
        Ok("数据加载成功 ✓".to_string())
    } else {
        Err("网络错误 ✗".to_string())
    }
}

/// 带指数退避的重试
const MAX_RETRIES: u32 = 4;

async fn fetch_with_retry() -> (Result<String, String>, u32) {
    let mut attempts = 0u32;
    loop {
        attempts += 1;
        let result = unreliable_request().await;
        match result {
            Ok(_) => return (result, attempts),
            Err(_) if attempts < MAX_RETRIES => {
                // 指数退避: 200ms, 400ms, 800ms, ...
                let delay = 200 * 2u64.pow(attempts - 1);
                sleep(delay).await;
                continue;
            }
            Err(_) => return (result, attempts),
        }
    }
}

#[component]
fn Exercise() -> impl IntoView {
    let data = RwSignal::new(None::<String>);
    let error = RwSignal::new(None::<String>);
    let attempts = RwSignal::new(0u32);
    let loading = RwSignal::new(false);

    let start_request = move || {
        loading.set(true);
        error.set(None);
        data.set(None);
        spawn_local(async move {
            let (result, attempt_count) = fetch_with_retry().await;
            attempts.set(attempt_count);
            loading.set(false);
            match result {
                Ok(val) => data.set(Some(val)),
                Err(e) => error.set(Some(e)),
            }
        });
    };

    view! {
        <div>
            <h2>"e197: 自动重试 —— 指数退避"</h2>
            <p>"每次请求有 60% 概率失败, 最多重试 4 次。"</p>
            <button on:click=move |_| start_request() disabled=move || loading.get()>
                {move || if loading.get() { "请求中…" } else { "发起请求" }}
            </button>
            <p>
                "尝试次数: " {move || attempts.get()}
                {move || if loading.get() { " (重试中…)" } else { "" }}
            </p>
            {move || data.get().map(|d| view! { <p style="color:green">"结果: " {d}</p> })}
            {move || error.get().map(|e| view! { <p style="color:red">"错误: " {e}</p> })}
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(Exercise);
}

// ============================================================
// 参考答案 (思考后再看!)
// ============================================================
// <details>
// <summary>点击展开答案</summary>
//
// ### 核心思路
// `loop` + `attempts` 计数，每次失败后按 `2^(attempt-1)*200ms` 延迟重试。
// 使用 `AtomicU32` 伪随机替代 `js_sys::Math::random()`。
//
// ### 关键代码
// ```rust
// async fn fetch_with_retry() -> (Result<String, String>, u32) {
//     let mut attempts = 0;
//     loop {
//         attempts += 1;
//         match unreliable_request().await {
//             Ok(_) => return (result, attempts),
//             Err(_) if attempts < 4 => {
//                 sleep(200 * 2u64.pow(attempts - 1)).await;
//                 continue;
//             }
//             Err(e) => return (Err(e), attempts),
//         }
//     }
// }
// ```
//
// ### 知识点
// - 指数退避: 避免对服务器造成二次压力
// - `spawn_local` 在 WASM 中启动异步任务
// - 结合 RwSignal 显示重试进度
//
// </details>
