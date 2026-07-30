// ============================================================
// Exercise e339 — Answer: Webhook Receiver & HMAC Verification
//
// Core: #[server] webhook handler, HMAC-SHA256 signature verification
// ============================================================

use leptos::prelude::*;
use leptos::prelude::ServerFnError;

const WEBHOOK_SECRET: &str = "your-webhook-secret-here";

#[derive(Debug, Clone)]
enum WebhookEvent {
    Push { branch: String, commit_count: u32 },
    PullRequest { action: String, title: String },
    Issues { action: String, title: String },
}

fn verify_hmac_signature(
    payload: &[u8],
    signature: &str,
    secret: &str,
) -> Result<(), String> {
    use hmac::{Hmac, Mac};
    use sha2::Sha256;
    use subtle::ConstantTimeEq;

    let mut mac = Hmac::<Sha256>::new_from_slice(secret.as_bytes())
        .map_err(|e| format!("HMAC key error: {}", e))?;
    mac.update(payload);
    let expected = format!("sha256={}", hex::encode(mac.finalize().into_bytes()));

    if signature.as_bytes().ct_eq(expected.as_bytes()).into() {
        Ok(())
    } else {
        Err("HMAC signature mismatch".to_string())
    }
}

#[server(WebhookReceive, "/api/webhook")]
pub async fn webhook_receive(payload: String, signature: String) -> Result<String, ServerFnError> {
    verify_hmac_signature(payload.as_bytes(), &signature, WEBHOOK_SECRET)
        .map_err(|e| ServerFnError::new(e))?;

    let event_type = if payload.contains("\"push\"") { "push" }
        else if payload.contains("\"pull_request\"") { "pull_request" }
        else if payload.contains("\"issues\"") { "issues" }
        else { "unknown" };

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
            <h1>"Webhook Receiver — HMAC Signature Verification"</h1>

            <section>
                <h2>"HMAC-SHA256 Verification"</h2>
                <pre>{HMAC_CODE}</pre>
            </section>

            <section>
                <h2>"Supported Webhook Events"</h2>
                <ul>
                    <li><strong>"Push"</strong>": Code push to branch"</li>
                    <li><strong>"Pull Request"</strong>": PR opened/updated/merged"</li>
                    <li><strong>"Issues"</strong>": Issue created/commented/closed"</li>
                </ul>
            </section>

            <section>
                <h2>"Server Endpoint"</h2>
                <pre>{"\
#[server(WebhookReceive, \"/api/webhook\")]
pub async fn webhook_receive(
    payload: String,
    signature: String,
) -> Result<String, ServerFnError> {
    verify_hmac_signature(&payload, &signature, WEBHOOK_SECRET)?;
    Ok(format!(\"Event: {event_type}\"))
}"}</pre>
            </section>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
