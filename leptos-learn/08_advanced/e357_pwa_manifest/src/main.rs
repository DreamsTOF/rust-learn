// ============================================================
// 练习 e357: PWA manifest — PWA manifest 配置和安装提示
//
// 核心知识点:
//   - 动态生成 manifest JSON 内容
//   - 通过 <link rel="manifest"> 注入到页面
//   - 监听 beforeinstallprompt 事件
//
// 难度: ⭐⭐ (关键位置有 TODO，补全约 50%)
// ============================================================

use leptos::prelude::*;

/// 生成 PWA manifest JSON 字符串
fn generate_manifest_json() -> String {
    // TODO: 构造一个完整的 PWA manifest JSON
    // 包含: name, short_name, description, start_url, display, theme_color, background_color, icons
    // 注意: 勿在 raw string r#"..."# 中包含 "# 序列，否则会提前结束字符串
    // 改用 push_str 拼接
    let mut s = String::from("{\n");
    s.push_str("  \"name\": \"Leptos PWA 演示\",\n");
    s.push_str("  \"short_name\": \"L-PWA\",\n");
    s.push_str("  \"description\": \"Leptos 框架 PWA 练习示例\",\n");
    s.push_str("  \"start_url\": \"/\",\n");
    s.push_str("  \"display\": \"standalone\",\n");
    s
}

#[component]
fn Exercise() -> impl IntoView {
    // TODO: 创建信号 tracking 安装提示是否可用
    // let (can_install, set_can_install) = signal(false);

    // TODO: 创建信号 tracking 应用是否已安装
    // let (is_installed, set_is_installed) = signal(false);

    // TODO: 在 Effect::new 中注入 manifest 链接到 <head>
    // 1. 使用 document().create_element("link")
    // 2. 设置 rel="manifest", href="data:application/json,..."
    // 3. 追加到 document().head()

    // TODO: 使用 window_event_listener 监听 beforeinstallprompt

    view! {
        <div style="max-width: 600px; margin: 2rem auto; font-family: sans-serif; text-align: center;">
            <h1>"📱 PWA 安装"</h1>

            <div style="padding: 1rem; background: #e3f2fd; border-radius: 8px; margin: 1rem 0;">
                <p>"这是一个 PWA 演示应用，您可以将其安装到设备上。"</p>
                // TODO: 显示安装按钮 (当可安装且未安装时)
                // <button on:click=move |_| { /* 触发安装 */ }>"安装应用"</button>
            </div>

            // TODO: 当已安装时显示提示
            // <div style="padding: 1rem; background: #e8f5e9; border-radius: 8px;">
            //     "✅ 感谢安装！"
            // </div>

            <div style="margin-top: 2rem; padding: 1rem; background: #f5f5f5; border-radius: 8px;">
                <h2>"📋 Manifest 配置"</h2>
                <pre style="text-align: left; background: #263238; color: #e0e0e0; padding: 1rem; border-radius: 4px; overflow-x: auto;">
                    {generate_manifest_json()}
                </pre>
            </div>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
