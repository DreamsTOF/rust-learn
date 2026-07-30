// ============================================================
// Exercise 291 - Answer
// ============================================================

use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <div>
            <h2>"cargo-leptos SSR 项目结构"</h2>
            <p>"cargo-leptos 是 Leptos 官方推荐的 SSR 项目脚手架工具。"</p>
            <ul>
                <li>"src/main.rs — 服务器入口，启动 Leptos 服务器"</li>
                <li>"src/lib.rs — 应用入口，定义路由和服务器函数"</li>
                <li>"src/app.rs — 应用组件，定义页面 UI"</li>
            </ul>
            <p>"运行: " <code>"cargo leptos watch"</code></p>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
