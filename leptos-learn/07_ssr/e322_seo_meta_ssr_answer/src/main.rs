// ============================================================
// Exercise e322 — seo_meta_ssr — Answer
//
// Core: Dynamic <meta> tags, Open Graph, SEO for SSR
// ============================================================

use leptos::prelude::*;
use leptos::meta::*;

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <div>
            <h2>"SEO & Meta Tags (SSR)"</h2>

            // Standard meta tags
            <Meta name="description" content="Leptos SSR with dynamic Open Graph tags" />
            <Meta name="keywords" content="leptos, ssr, seo, rust" />

            // Open Graph tags for social sharing
            <Meta property="og:title" content="Leptos SSR Example" />
            <Meta property="og:description" content="Demonstrating dynamic Meta components in Leptos 0.8 SSR" />
            <Meta property="og:image" content="https://example.com/og-image.png" />
            <Meta property="og:type" content="website" />

            <p>"检查 <head> 中的 <meta> 标签（打开浏览器 DevTools）"</p>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
