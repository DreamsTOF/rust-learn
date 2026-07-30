// ============================================================
// 参考答案 e378: WebAuthn 密码匙认证
//
// 模拟 WebAuthn 注册与认证流程，展示公钥凭证数据结构
// ============================================================

use leptos::prelude::*;

/// 模拟凭证注册 — 实际项目中使用 navigator.credentials.create()
fn simulate_register() -> String {
    r#"{
  "simulated": true,
  "id": "mock-credential-id-KL5f2g8H",
  "type": "public-key",
  "rawIdHex": "a1b2c3d4e5f60718293a4b5c6d7e8f9001a2b3c4d5e6f708192a3b4c5d6e7f809",
  "rp": { "name": "Leptos Learn" },
  "user": { "name": "test@example.com", "displayName": "Test User" },
  "algorithm": "ES256 (alg: -7)",
  "note": "在生产环境的 HTTPS 下，此数据由浏览器 WebAuthn API 生成"
}"#.to_string()
}

/// 模拟认证响应 — 实际项目中使用 navigator.credentials.get()
fn simulate_authenticate() -> String {
    r#"{
  "simulated": true,
  "id": "mock-credential-id-KL5f2g8H",
  "type": "public-key",
  "userHandle": "user_001",
  "signature": "3045022100f42a8c5a8d5a8f7c8b9a0b1c2d3e4f5061728394a5b6c7d8e9f00112233445566",
  "clientDataJSON": "eyJ0eXBlIjoid2ViYXV0aG4uZ2V0IiwiY2hhbGxlbmdlIjoiYTFiMmMzZDQifQ",
  "authenticatorData": "d496d5a5c5a8f7c8b9a0b1c2d3e4f5061728394a5b6c7d8e9f00112233445566000000001",
  "note": "认证通过后服务端需验证签名与挑战值"
}"#.to_string()
}

#[component]
fn Exercise() -> impl IntoView {
    let status = RwSignal::new("准备就绪，点击下方按钮查看 WebAuthn 流程模拟。".to_string());
    let credential_info = RwSignal::new(String::new());

    let handle_register = move |_| {
        status.set("📝 模拟注册：生成公钥凭证...".to_string());
        credential_info.set(simulate_register());
    };

    let handle_authenticate = move |_| {
        status.set("🔓 模拟认证：验证签名...".to_string());
        credential_info.set(simulate_authenticate());
    };

    view! {
        <div style="padding: 20px; max-width: 600px; margin: 0 auto; font-family: system-ui, sans-serif;">
            <h2>"🔐 WebAuthn 密码匙认证"</h2>
            <p style="color: #666;">
                "WebAuthn 是一种无密码认证标准，使用公钥加密替代传统密码。"
                "用户通过生物识别（指纹/面部）或硬件安全密钥完成认证。"
            </p>

            <div style="border: 1px solid #ddd; padding: 16px; border-radius: 8px; margin: 16px 0;
                        background: #f5f5f5;">
                <p><strong>"状态:"</strong> {move || status.get()}</p>
            </div>

            <div style="display: flex; gap: 12px; margin: 16px 0;">
                <button
                    style="flex: 1; background: #1976d2; color: white; border: none; padding: 12px 24px;
                           border-radius: 6px; cursor: pointer; font-size: 14px;"
                    on:click=handle_register
                >
                    "📝 注册凭证 (模拟)"
                </button>
                <button
                    style="flex: 1; background: #388e3c; color: white; border: none; padding: 12px 24px;
                           border-radius: 6px; cursor: pointer; font-size: 14px;"
                    on:click=handle_authenticate
                >
                    "🔓 认证 (模拟)"
                </button>
            </div>

            <Show when=move || !credential_info.get().is_empty()>
                <div style="border: 1px solid #ccc; padding: 16px; border-radius: 8px;
                            background: #fff; margin-top: 16px;">
                    <h3>"凭证信息"</h3>
                    <pre style="white-space: pre-wrap; word-break: break-all; font-size: 13px;
                               background: #fafafa; padding: 12px; border-radius: 4px; border: 1px solid #eee;">
                        {move || credential_info.get()}
                    </pre>
                </div>
            </Show>

            <div style="margin-top: 20px; padding: 16px; border: 1px solid #e0e0e0; border-radius: 8px;
                        background: #fafafa;">
                <h3>"WebAuthn 流程说明"</h3>
                <ol style="line-height: 1.8; padding-left: 20px;">
                    <li><strong>注册 (Registration):</strong> 调用 <code>navigator.credentials.create()</code>
                        " — 浏览器弹出系统认证对话框（指纹/Face ID/安全密钥）。"
                    </li>
                    <li><strong>认证 (Authentication):</strong> 调用 <code>navigator.credentials.get()</code>
                        " — 浏览器要求用户验证身份，返回签名断言。"
                    </li>
                    <li><strong>验证:</strong> "服务端使用注册时存储的公钥验证签名，确认身份。"
                    </li>
                </ol>
                <p style="margin-top: 8px; color: #888; font-size: 13px;">
                    "💡 WebAuthn 需要 HTTPS 环境。本练习以模拟数据展示数据结构与流程。"
                </p>
            </div>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
