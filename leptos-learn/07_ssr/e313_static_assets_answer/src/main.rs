// ============================================================
// 练习 e313: 静态资源管理 — 参考答案
//
// 核心知识点:
//   - 资源哈希（asset hashing）实现长期缓存与即时失效
//   - 不同资源类型的 Cache-Control 策略
//   - SSR 中静态文件服务配置（axum / actix）
// ============================================================

use leptos::prelude::*;

/// 模拟资源哈希表 — 原始文件名 → 哈希版本文件名
const HASHED_ASSETS: &[(&str, &str)] = &[
    ("main.css", "main-a1b2c3d4.css"),
    ("app.js",   "app-e5f6g7h8.js"),
    ("logo.png", "logo-i9j0k1l2.png"),
];

/// 不同类型资源的 Cache-Control 策略
const CACHE_POLICIES: &[(&str, &str, &str)] = &[
    ("HTML",    "index.html",            "no-cache"),
    ("CSS/JS",  "*.css, *.js",          "public, max-age=31536000, immutable"),
    ("图片",    "*.png, *.jpg",          "public, max-age=86400"),
    ("字体",    "*.woff2",               "public, max-age=31536000, immutable"),
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
                    <For each=move || HASHED_ASSETS key=|(orig, _)| *orig let:(orig, hashed)>
                        <tr>
                            <td style="padding: 6px; border-bottom: 1px solid #eee;">{*orig}</td>
                            <td style="padding: 6px; border-bottom: 1px solid #eee;"><code>{*hashed}</code></td>
                        </tr>
                    </For>
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
                    <For each=move || CACHE_POLICIES key=|(type_, _, _)| *type_ let:(type_, pattern, policy)>
                        <tr>
                            <td style="padding: 6px; border-bottom: 1px solid #eee;">{*type_}</td>
                            <td style="padding: 6px; border-bottom: 1px solid #eee;"><code>{*pattern}</code></td>
                            <td style="padding: 6px; border-bottom: 1px solid #eee;"><code>{*policy}</code></td>
                        </tr>
                    </For>
                </table>
            </section>

            <hr/>

            <section>
                <h3>"3. SSR 静态文件服务配置"</h3>
                <pre style="background: #f5f5f5; padding: 12px; border-radius: 6px; overflow-x: auto;">
{r#"// axum 示例 — 静态文件中间件 + 缓存头
use axum::Router;
use tower_http::services::ServeDir;
use tower_http::set_header::SetResponseHeaderLayer;
use axum::http::{header, HeaderValue};

let app = Router::new()
    .nest_service(
        "/pkg",
        ServeDir::new("pkg").precompressed_gzip(),
    )
    .layer(SetResponseHeaderLayer::overriding(
        header::CACHE_CONTROL,
        HeaderValue::from_static("public, max-age=31536000, immutable"),
    ));"#}
                </pre>
                <p>
                    "Leptos SSR 应用中，"
                    <code>"leptos_axum::file_and_error_handler"</code>
                    " 自动处理静态文件服务和错误回退。"
                    "构建工具（cargo-leptos）自动生成哈希文件名并注入到 HTML。"
                </p>
            </section>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
