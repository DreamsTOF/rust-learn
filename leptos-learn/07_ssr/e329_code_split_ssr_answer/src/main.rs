// ============================================================
// Exercise e329 — Answer: Code Splitting & SSR
//
// Core: Dynamic imports, lazy loading, chunk splitting
// ============================================================

use leptos::prelude::*;
use leptos::task::spawn_local;

fn LazyHeavyWidget() -> impl IntoView {
    let loaded = RwSignal::new(false);
    let content = RwSignal::new(String::new());

    spawn_local(async move {
        // Simulate a dynamic import / chunk load delay
        gloo_timers::future::sleep(std::time::Duration::from_millis(800)).await;
        content.set("重型组件已加载 — 这部分代码来自单独的 chunk。".to_string());
        loaded.set(true);
    });

    view! {
        <div style="border:2px solid #2196F3;padding:1rem;border-radius:8px;">
            {move || {
                if loaded.get() {
                    view! { <p><strong>{content.get()}</strong></p> }.into_any()
                } else {
                    view! { <p>"⏳ 加载中..."</p> }.into_any()
                }
            }}
        </div>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <div>
            <h1>"Code Splitting (SSR)"</h1>
            <p>"下方组件模拟动态导入 — 初始渲染时不包含其代码。"</p>
            {LazyHeavyWidget()}
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
