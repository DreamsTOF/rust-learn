// ============================================================
// 练习 e357: PWA manifest — PWA manifest 配置和安装提示
//
// 核心知识点:
//   - 动态生成 manifest JSON 内容并注入页面
//   - 管理 PWA 安装流程状态
//   - 使用 leptos 的 document() API 操作 DOM
//
// 难度: ⭐⭐
// ============================================================

use leptos::prelude::*;

/// 生成 PWA manifest JSON 字符串
fn generate_manifest_json() -> String {
    let mut s = String::from("{\n");
    s.push_str("  \"name\": \"Leptos PWA 演示\",\n");
    s.push_str("  \"short_name\": \"L-PWA\",\n");
    s.push_str("  \"description\": \"Leptos 框架 PWA 练习示例\",\n");
    s.push_str("  \"start_url\": \"/\",\n");
    s.push_str("  \"display\": \"standalone\",\n");
    s.push_str("  \"theme_color\": \"#1a73e8\",\n");
    s.push_str("  \"background_color\": \"#ffffff\",\n");
    s.push_str("  \"icons\": [\n");
    s.push_str("    { \"src\": \"icon-192.png\", \"sizes\": \"192x192\", \"type\": \"image/png\" },\n");
    s.push_str("    { \"src\": \"icon-512.png\", \"sizes\": \"512x512\", \"type\": \"image/png\" }\n");
    s.push_str("  ]\n");
    s.push_str("}\n");
    s
}

/// 将 manifest JSON 编码为 data URI 并注入 <head>
fn inject_manifest(manifest_json: &str) {
    // 对 JSON 中的特殊字符进行 URL 编码
    let mut encoded = String::new();
    for c in manifest_json.chars() {
        match c {
            ' ' => encoded.push_str("%20"),
            '\n' => {}
            '"' => encoded.push_str("%22"),
            '{' => encoded.push_str("%7B"),
            '}' => encoded.push_str("%7D"),
            ',' => encoded.push_str("%2C"),
            ':' => encoded.push_str("%3A"),
            other => encoded.push(other),
        }
    }

    let data_uri = format!("data:application/json,{}", encoded);

    let doc = document();
    if let Some(head) = doc.head() {
        let link = doc.create_element("link").unwrap();
        let _ = link.set_attribute("rel", "manifest");
        let _ = link.set_attribute("href", &data_uri);
        let _ = head.append_child(&link);
    }
}

#[component]
fn Exercise() -> impl IntoView {
    let (can_install, set_can_install) = signal(true);
    let (is_installed, set_is_installed) = signal(false);

    // 注入 manifest 链接到页面 <head>
    inject_manifest(&generate_manifest_json());

    // 模拟安装：点击按钮后标记为已安装
    let handle_install = move |_| {
        set_is_installed.set(true);
        set_can_install.set(false);
    };

    view! {
        <div style="max-width: 640px; margin: 2rem auto; font-family: system-ui, sans-serif; text-align: center;">
            <h1>"📱 PWA 安装演示"</h1>

            <div style="padding: 1.5rem; background: #e3f2fd; border-radius: 12px; margin: 1rem 0;">
                {move || {
                    if is_installed.get() {
                        view! {
                            <div style="padding: 1rem; background: #e8f5e9; color: #2e7d32; border-radius: 8px; font-weight: bold;">
                                "✅ 感谢安装！应用已添加到主屏幕"
                            </div>
                        }.into_any()
                    } else if can_install.get() {
                        view! {
                            <>
                                <p>"此应用支持 PWA 安装，可添加到主屏幕。"</p>
                                <button on:click=handle_install
                                    style="padding: 0.75rem 2rem; font-size: 1rem; background: #1a73e8; color: white; border: none; border-radius: 8px; cursor: pointer; margin-top: 0.5rem; font-weight: 500;">
                                    "📲 安装应用"
                                </button>
                            </>
                        }.into_any()
                    } else {
                        view! {
                            <p style="color: #888; font-style: italic;">
                                "💡 在支持的浏览器（Chrome/Edge）中打开，会出现安装按钮"
                            </p>
                        }.into_any()
                    }
                }}
            </div>

            <div style="margin-top: 2rem; padding: 1.25rem; background: #f5f5f5; border-radius: 12px; text-align: left;">
                <h2>"📋 Manifest JSON"</h2>
                <pre style="background: #263238; color: #e0e0e0; padding: 1rem; border-radius: 8px; overflow-x: auto; font-size: 0.8rem; line-height: 1.5;">
                    {generate_manifest_json()}
                </pre>
            </div>

            <div style="margin-top: 1.5rem; padding: 1rem; background: #fff8e1; border-radius: 8px; font-size: 0.85rem; color: #f57f17;">
                <strong>"🔗 注入的 manifest 链接："</strong>
                <code style="display: block; margin-top: 0.25rem; word-break: break-all;">
                    {"<link rel=\"manifest\" href=\"data:application/json,...\">"}
                </code>
            </div>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
