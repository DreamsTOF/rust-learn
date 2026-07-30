// ============================================================
// 练习 e378: WebAuthn 密码匙认证 — 模拟注册与认证流程
//
// 核心知识点:
//   - WebAuthn 公钥凭证体系的基本概念
//   - 注册流程：navigator.credentials.create()
//   - 认证流程：navigator.credentials.get()
//   - 公钥凭证数据结构
//
// 难度: ⭐⭐ (需补全模拟数据生成和流程展示逻辑，约 50%)
// ============================================================

use leptos::prelude::*;

/// 模拟凭证注册 — TODO 1: 补全返回的 JSON 字符串，包含 id、type、rawIdHex
fn simulate_register() -> String {
    // TODO 1: 返回一个包含模拟凭证信息的 JSON 字符串
    // 应包含字段: simulated, id, type, rawIdHex, rp, user
    String::new() // placeholder
}

/// 模拟认证响应 — TODO 2: 补全返回的 JSON 字符串，包含签名数据
fn simulate_authenticate() -> String {
    // TODO 2: 返回一个包含模拟认证响应的 JSON 字符串
    // 应包含字段: simulated, id, type, userHandle, signature
    String::new() // placeholder
}

#[component]
fn Exercise() -> impl IntoView {
    let status = RwSignal::new("准备就绪，点击按钮查看 WebAuthn 流程模拟。".to_string());
    let credential_info = RwSignal::new(String::new());

    let handle_register = move |_| {
        status.set("📝 模拟注册：生成公钥凭证...".to_string());
        // TODO 3: 调用 simulate_register() 并设置到 credential_info
    };

    let handle_authenticate = move |_| {
        status.set("🔓 模拟认证：验证签名...".to_string());
        // TODO 4: 调用 simulate_authenticate() 并设置到 credential_info
    };

    view! {
        <div style="padding: 20px; max-width: 600px; margin: 0 auto; font-family: system-ui, sans-serif;">
            <h2>"🔐 WebAuthn 密码匙认证"</h2>
            <p style="color: #666;">
                "WebAuthn 是一种无密码认证标准，使用公钥加密替代传统密码。"
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
                        // TODO 5: 显示 credential_info 的内容
                    </pre>
                </div>
            </Show>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
