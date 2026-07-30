// ============================================================
// 练习 e280: 暗黑模式 — 参考答案
//
// 核心知识点:
//   - class:xxx 动态切换 CSS 类名
//   - CSS 变量（自定义属性）实现主题色定义
//   - @media (prefers-color-scheme: dark) 媒体查询
//   - <Style/> 组件定义全局样式
// ============================================================

use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    let (dark, set_dark) = signal(false);

    view! {
        <div class:dark={move || dark.get()}>
            <style>
                ":root { --bg: #ffffff; --text: #333333; --primary: #0066cc; }"
                ".dark { --bg: #1a1a2e; --text: #e0e0e0; --primary: #66b3ff; }"
                "@media (prefers-color-scheme: dark) {"
                "  :root { --bg: #1a1a2e; --text: #e0e0e0; --primary: #66b3ff; }"
                "}"
                "body { background-color: var(--bg); color: var(--text); transition: all 0.3s ease; font-family: sans-serif; }"
            </style>

            <div style:background-color="var(--bg)" style:color="var(--text)">
                <h1>"暗黑模式"</h1>
                <button
                    on:click=move |_| set_dark(!dark.get())
                    style:background-color="var(--primary)"
                    style:color="white"
                    style:border="none"
                    style:padding="0.5rem 1rem"
                    style:border-radius="4px"
                    style:cursor="pointer"
                >
                    {move || if dark.get() { "切换到亮色模式" } else { "切换到暗黑模式" }}
                </button>
            </div>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
