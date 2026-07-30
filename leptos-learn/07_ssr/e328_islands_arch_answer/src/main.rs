// ============================================================
// Exercise e328 — Answer: Islands Architecture
//
// Core: Islands pattern, interactive islands, lazy hydration
// ============================================================

use leptos::prelude::*;

/// 岛屿组件 — 在服务端渲染为静态 HTML，
/// 客户端独立水合为交互式小岛
fn CounterIsland() -> impl IntoView {
    let count = RwSignal::new(0);
    view! {
        <div style="border:2px solid #4caf50;padding:1rem;border-radius:8px;">
            <p>
                "计数: "
                <strong>{move || count.get()}</strong>
            </p>
            <button on:click=move |_| count.update(|n| *n += 1)>"+"</button>
            <button on:click=move |_| count.update(|n| *n -= 1)>"-"</button>
            <button on:click=move |_| count.set(0)>"重置"</button>
        </div>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <div>
            <h1>"Islands 架构"</h1>
            <p>
                "下方是一个独立的交互岛屿。"
                "服务端只输出静态 HTML，"
                "客户端只水合此 <div> 内的 JS。"
            </p>
            {CounterIsland()}
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
