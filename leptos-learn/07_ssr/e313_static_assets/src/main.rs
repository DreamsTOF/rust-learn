// ============================================================
// 练习 e313: 静态资源管理 — 文件服务、资源哈希与缓存策略
//
// 核心知识点:
//   - SSR 中静态文件服务配置（axum::StaticFileDir / actix_files）
//   - 资源哈希（asset hashing）实现缓存失效
//   - Cache-Control 头配置策略
//
// 难度: ⭐⭐ (关键 TODOs，约 50% 已补全)
// ============================================================

use leptos::prelude::*;

/// 模拟资源哈希表 — 文件名 → 哈希版本
const HASHED_ASSETS: &[(&str, &str)] = &[
    ("main.css", "main-a1b2c3d4.css"),
    ("app.js",   "app-e5f6g7h8.js"),
    ("logo.png", "logo-i9j0k1l2.png"),
];

/// 模拟 Cache-Control 策略配置
///
/// 不同资源类型使用不同的缓存策略
// TODO: 补全策略描述
const CACHE_POLICIES: &[(&str, &str, &str)] = &[
    ("HTML",    "index.html",   "no-cache"),
    ("CSS/JS",  "*.css, *.js",  "public, max-age=31536000, immutable"),
    ("图片",    "*.png, *.jpg", "public, max-age=86400"),
    // TODO: 添加字体资源策略 (woff2, "public, max-age=31536000, immutable")
    // ("字体",    "*.woff2",      "public, max-age=31536000, immutable"),
];

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <div style="max-width: 640px; margin: 24px auto; font-family: system-ui, sans-serif;">
            <h2>"📦 静态资源管理"</h2>

            <section>
                <h3>"1. 资源哈希（Asset Hashing）"</h3>
                <p>"文件内容变化时哈希值改变，浏览器缓存自动失效："</p>
                <table style="width: 100%; border-collapse: collapse;">
                    <tr>
                        <th style="text-align: left; border-bottom: 1px solid #ccc; padding: 6px;">"原始文件"</th>
                        <th style="text-align: left; border-bottom: 1px solid #ccc; padding: 6px;">"哈希版本"</th>
                    </tr>
                    // TODO: 使用 For 遍历 HASHED_ASSETS 渲染表格行
                </table>
            </section>

            <hr/>

            <section>
                <h3>"2. Cache-Control 策略"</h3>
                <p>"不同资源类型应用不同的缓存头："</p>
                <table style="width: 100%; border-collapse: collapse;">
                    <tr>
                        <th style="text-align: left; border-bottom: 1px solid #ccc; padding: 6px;">"类型"</th>
                        <th style="text-align: left; border-bottom: 1px solid #ccc; padding: 6px;">"匹配模式"</th>
                        <th style="text-align: left; border-bottom: 1px solid #ccc; padding: 6px;">"Cache-Control"</th>
                    </tr>
                    <tr>
                        <td>"HTML"</td>
                        <td>"index.html"</td>
                        <td>"no-cache"</td>
                    </tr>
                    // TODO: 添加 CSS/JS 和图片行
                </table>
            </section>

            <hr/>

            <section>
                <h3>"3. SSR 静态文件服务配置"</h3>
                <pre style="background: #f5f5f5; padding: 12px; border-radius: 6px; overflow-x: auto;">
{r#"// axum 示例 — 静态文件中间件
use axum::routing::get_service;
use tower_http::services::ServeDir;

async fn main() {
    let app = Router::new()
        .nest_service(
            "/pkg",
            get_service(ServeDir::new("pkg"))
                .handle_error(|e| ...),
        )
        // 设置缓存头
        .layer(SetResponseHeaderLayer::overriding(
            header::CACHE_CONTROL,
            HeaderValue::from_static("public, max-age=31536000, immutable"),
        ));
}"#}
                </pre>
                // TODO: 添加说明 — leptos_axum::file_and_error_handler 的自动静态文件服务
                <p>"Leptos 提供 " <code>"leptos_axum::file_and_error_handler"</code> " 自动处理静态文件。"</p>
            </section>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
