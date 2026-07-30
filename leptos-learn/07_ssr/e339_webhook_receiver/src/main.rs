// ============================================================
// 练习 e339: Webhook Receiver — HMAC 签名验证
//
// 核心知识点:
//   - #[server] 宏: 定义服务端 webhook 端点
//   - HMAC-SHA256 签名验证: 确保请求来自可信来源
//   - webhook 负载解析与事件路由
//
// 难度: ⭐⭐⭐ (少量 TODO)
// ============================================================

use leptos::prelude::*;
use leptos::prelude::ServerFnError;

// Webhook 共享密钥（生产环境应通过环境变量注入）
const WEBHOOK_SECRET: &str = "your-webhook-secret-here";

/// Webhook 事件类型枚举
#[derive(Debug, Clone)]
enum WebhookEvent {
    /// 代码推送事件
    Push { branch: String, commit_count: u32 },
    /// Pull Request 事件
    PullRequest { action: String, title: String },
    /// Issues 事件
    Issues { action: String, title: String },
}

// TODO: 实现 HMAC-SHA256 签名验证
// ⭐⭐⭐ 使用 hmac + sha2 crate 验证签名，
// 返回 Result<(), String>，失败时返回错误信息。
// 实际代码使用 constant-time comparison 防止时序攻击。
fn verify_hmac_signature(
    payload: &[u8],
    signature: &str,
    secret: &str,
) -> Result<(), String> {
    // ⭐⭐⭐ TODO: 补全 HMAC-SHA256 验证逻辑
    // 提示: 使用 Hmac::<Sha256>::new_from_slice(secret.as_bytes())
    //       计算 HMAC，与 "sha256=" + hex(hmac_result) 比较
    //       使用 subtle::ConstantTimeEq 做常量时间比较
    //
    // 示意实现（占位）:
    if signature.starts_with("sha256=") && signature.len() > 7 {
        Ok(())
    } else {
        Err("Invalid signature format".to_string())
    }
}

// Webhook 接收端点
#[server(WebhookReceive, "/api/webhook")]
pub async fn webhook_receive(payload: String, signature: String) -> Result<String, ServerFnError> {
    // 1. 验证 HMAC 签名
    verify_hmac_signature(payload.as_bytes(), &signature, WEBHOOK_SECRET)
        .map_err(|e| ServerFnError::new(e))?;

    // 2. 解析事件类型（示意）
    let event_type = if payload.contains("\"push\"") { "push" }
        else if payload.contains("\"pull_request\"") { "pull_request" }
        else if payload.contains("\"issues\"") { "issues" }
        else { "unknown" };

    // 3. 返回处理结果
    Ok(format!("Webhook received and verified. Event: {}", event_type))
}

#[component]
fn Exercise() -> impl IntoView {
    const HMAC_CODE: &str = "\
use hmac::{Hmac, Mac};
use sha2::Sha256;
use subtle::ConstantTimeEq;

fn verify_hmac(payload: &[u8], sig: &str, secret: &str) -> Result<(), String> {
    let mut mac = Hmac::<Sha256>::new_from_slice(secret.as_bytes())
        .map_err(|e| format!(\"Key error: {e}\"))?;
    mac.update(payload);
    let expected = format!(\"sha256={}\",
        hex::encode(mac.finalize().into_bytes()));
    if sig.as_bytes().ct_eq(expected.as_bytes()).into() {
        Ok(())
    } else {
        Err(\"HMAC mismatch\".to_string())
    }
}";

    view! {
        <div>
            <h1>"Webhook Receiver — HMAC 签名验证"</h1>

            <section>
                <h2>"HMAC-SHA256 验证"</h2>
                <pre>{HMAC_CODE}</pre>
            </section>

            <section>
                <h2>"支持的 Webhook 事件"</h2>
                <ul>
                    <li><strong>"Push"</strong>": 代码推送至分支"</li>
                    <li><strong>"Pull Request"</strong>": PR 创建/更新/合并"</li>
                    <li><strong>"Issues"</strong>": Issue 创建/评论/关闭"</li>
                </ul>
            </section>

            <section>
                <h2>"服务器端点"</h2>
                <pre>{"\
#[server(WebhookReceive, \"/api/webhook\")]
pub async fn webhook_receive(
    payload: String,
    signature: String,
) -> Result<String, ServerFnError> {
    verify_hmac_signature(&payload, &signature, SECRET)?;
    // 解析事件并处理...
    Ok(format!(\"Event: {event_type}\"))
}"}</pre>
            </section>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
