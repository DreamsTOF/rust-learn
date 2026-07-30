// ============================================================
// 练习 e330: Cache Strategy — SSR 缓存策略
//
// 核心知识点:
//   - Cache-Control 响应头
//   - ETag / If-None-Match
//   - 响应缓存 & CDN 策略
//
// 难度: ⭐⭐⭐ (最小指引)
// ============================================================

use leptos::prelude::*;

// TODO: 定义缓存策略配置常量
// ⭐⭐⭐ 提示:
//   - 定义 Cache-Control 策略字符串
//   - 定义 ETag 生成逻辑描述字符串
//   - 定义 CDN 配置字符串（如 Cloudflare）

fn main() {
    mount_to_body(|| view! { <p>"练习 330 (cache_strategy)"</p> });
}
