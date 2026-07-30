// ============================================================
// 练习 e314: CSR mount_to_body vs SSR render_to_string — 参考答案
//
// 核心知识点:
//   - CSR: mount_to_body 客户端挂载，view! 宏在浏览器生成 DOM
//   - SSR: render_to_string 服务端渲染，输出完整 HTML 字符串
//   - 同步渲染流程: 组件定义 → render_to_string → HTTP 响应
//   - 适用场景: SEO、首屏性能、渐进增强
// ============================================================

use leptos::prelude::*;

/// 模拟 SSR render_to_string 的 HTML 输出
const SSR_HTML: &str = r#"<div id="ssr-root">
    <h2>SSR 渲染结果</h2>
    <p>此 HTML 在服务端生成，浏览器直接接收完整内容。</p>
    <ul>
        <li>SEO 友好 — 搜索引擎可抓取完整内容</li>
        <li>首屏快 — HTML 到达即可展示，无需等待 JS 加载</li>
        <li>渐进增强 — 水合后添加交互性</li>
    </ul>
</div>"#;

/// SSR 渲染流程的步骤定义
fn render_flow_example() -> Vec<(&'static str, &'static str, &'static str)> {
    vec![
        ("1. 组件定义", "#[component] fn App() -> impl IntoView", "声明组件结构，定义 view! 模板"),
        ("2. 渲染入口", "render_to_string(|| view! { <App/> })", "在服务端执行组件，生成 HTML 字符串"),
        ("3. HTML 响应", "HTTP Response → Content-Type: text/html", "浏览器接收完整 HTML，解析并展示"),
    ]
}

#[component]
fn Exercise() -> impl IntoView {
    let steps = render_flow_example();

    view! {
        <div style="max-width: 640px; margin: 24px auto; font-family: system-ui, sans-serif;">
            <h2>"🔄 CSR vs SSR 同步渲染"</h2>

            <section>
                <h3>"CSR (当前): mount_to_body"</h3>
                <p>
                    "当前页面使用 " <code>mount_to_body(Exercise)</code>
                    " — 组件在<strong>客户端</strong>编译、执行、挂载。"
                    "view! 宏在浏览器中生成 DOM 节点。"
                </p>
            </section>

            <hr/>

            <section>
                <h3>"SSR: render_to_string"</h3>
                <p>
                    "SSR 模式下使用 "
                    <code>leptos::ssr::render_to_string</code>
                    " — 组件在<strong>服务端</strong>执行，"
                    "输出完整 HTML 字符串直接返回给浏览器。"
                </p>

                <pre style="background: #f5f5f5; padding: 12px; border-radius: 6px; overflow-x: auto;">
{r#"// Cargo.toml 中启用 ssr feature
// leptos = { workspace = true, features = ["ssr"] }

use leptos::ssr::render_to_string;

let html = render_to_string(|| view! {
    <html>
        <head><title>SSR 页面</title></head>
        <body>
            <App/>
        </body>
    </html>
});

// html 是完整 HTML，可直接写入 HTTP 响应"#}
                </pre>
            </section>

            <hr/>

            <section>
                <h3>"SSR 渲染流程"</h3>
                <table style="width: 100%; border-collapse: collapse;">
                    <tr>
                        <th style="text-align: left; border-bottom: 1px solid #ccc; padding: 6px;">"步骤"</th>
                        <th style="text-align: left; border-bottom: 1px solid #ccc; padding: 6px;">"代码"</th>
                        <th style="text-align: left; border-bottom: 1px solid #ccc; padding: 6px;">"说明"</th>
                    </tr>
                    <For each=move || steps key=|(step, _, _)| *step let:(step, code, desc)>
                        <tr>
                            <td style="padding: 6px; border-bottom: 1px solid #eee;">{*step}</td>
                            <td style="padding: 6px; border-bottom: 1px solid #eee;"><code>{*code}</code></td>
                            <td style="padding: 6px; border-bottom: 1px solid #eee;">{*desc}</td>
                        </tr>
                    </For>
                </table>
            </section>

            <hr/>

            <section>
                <h3>"SSR 模拟输出"</h3>
                <pre style="background: #1e1e1e; color: #d4d4d4; padding: 12px; border-radius: 6px; overflow-x: auto;">
{SSR_HTML}
                </pre>
            </section>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
