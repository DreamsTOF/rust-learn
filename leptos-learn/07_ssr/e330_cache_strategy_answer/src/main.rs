// ============================================================
// Exercise e330 — Answer: Cache Strategy
//
// Core: Cache-Control, ETag, response caching, CDN
// ============================================================

use leptos::prelude::*;

const CACHE_CONTROL: &str = "\
# 静态资源（JS/CSS/图片）— 长期缓存
/pkg/*          Cache-Control: public, max-age=31536000, immutable

# SSR HTML 页面 — 短期缓存 + 校验
/               Cache-Control: public, max-age=60, stale-while-revalidate=600

# API 动态数据 — 不缓存
/api/*          Cache-Control: no-store, must-revalidate";

const ETAG_STRATEGY: &str = "\
ETag 生成策略:
  1. 对 SSR 响应内容的 hash (SHA-256) 取前 32 字符
  2. 客户端 If-None-Match → 服务端比对 → 304 Not Modified
  3. 页面内容未变时跳过完整渲染

  pseudocode:
    fn etag(body: &[u8]) -> String {
        let hash = sha256(body);
        format!(\"{:x}\", hash)[..32].to_string()
    }";

const CDN_CONFIG: &str = "\
# Cloudflare / Fastly CDN 配置要点

1. 边缘缓存: Cache-Control 指令透传
2. Purge: 部署后 POST /purge 清除 CDN 边缘缓存
3. Stale-while-revalidate: 过期后先返回陈旧内容，
   后台异步重新验证，减少用户等待
4. Tiered Cache: 上层缓存命中后回源请求减少 80%";

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <div>
            <h1>"SSR 缓存策略"</h1>
            <section>
                <h2>"Cache-Control"</h2>
                <pre>{CACHE_CONTROL}</pre>
            </section>
            <section>
                <h2>"ETag 策略"</h2>
                <pre>{ETAG_STRATEGY}</pre>
            </section>
            <section>
                <h2>"CDN 配置"</h2>
                <pre>{CDN_CONFIG}</pre>
            </section>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
