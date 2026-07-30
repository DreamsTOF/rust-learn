// ============================================================
// Leptos 练习项目 — 导航首页
//
// 本项目包含约 400+ 道 Leptos 练习题，覆盖从 view! 宏到
// 全栈实战的 8 章内容。每道题都是独立可运行的 Leptos CSR 应用。
//
// 使用方法:
//   cd <chapter>/<exercise>
//   trunk serve --open
// ============================================================

use leptos::prelude::*;

#[component]
fn App() -> impl IntoView {
    view! {
        <div style="max-width: 800px; margin: 0 auto; padding: 2rem; font-family: system-ui, sans-serif;">
            <h1>"Leptos 练习项目"</h1>
            <p>"欢迎！本项目包含 " <strong>"400+"</strong> " 道 Leptos 练习题，助你从零掌握 Leptos 全栈开发。"</p>

            <section>
                <h2>"快速开始"</h2>
                <pre style="background: #f5f5f5; padding: 1rem; border-radius: 4px;">
                    <code>{r#"# 进入任意练习目录
cd 01_basics/e01_hello_world

# 启动开发服务器
trunk serve --open"#}</code>
                </pre>
            </section>

            <section>
                <h2>"章节导航"</h2>
                <ul>
                    <li><strong>"第 1 章"</strong> " — 基础与环境 (e01-e20, 20 题)"</li>
                    <li><strong>"第 2 章"</strong> " — 响应式系统 (e21-e95, 75 题)"</li>
                    <li><strong>"第 3 章"</strong> " — 组件进阶 (e96-e155, 60 题)"</li>
                    <li><strong>"第 4 章"</strong> " — 异步与资源加载 (e156-e200, 45 题)"</li>
                    <li><strong>"第 5 章"</strong> " — 路由 (e201-e250, 50 题)"</li>
                    <li><strong>"第 6 章"</strong> " — 表单、样式与开发体验 (e251-e290, 40 题)"</li>
                    <li><strong>"第 7 章"</strong> " — SSR 与 Server Functions (e291-e345, 55 题)"</li>
                    <li><strong>"第 8 章"</strong> " — 高级模式 (e346-e385, 40 题)"</li>
                    <li><strong>"项目 A"</strong> " — ShopOS 全栈电商后台 (40 步)"</li>
                    <li><strong>"项目 B"</strong> " — NoteFlow 实时协作知识库 (40 步)"</li>
                </ul>
            </section>

            <section>
                <h2>"技术栈"</h2>
                <ul>
                    <li>"Leptos 0.9 + nightly Rust"</li>
                    <li>"Trunk (CSR) / cargo-leptos (SSR)"</li>
                    <li>"Thaw UI 组件库"</li>
                    <li>"leptos-use 响应式工具集"</li>
                </ul>
            </section>

            <footer style="margin-top: 3rem; color: #666; font-size: 0.9rem;">
                <p>"Happy Learning! 🚀"</p>
            </footer>
        </div>
    }
}

fn main() {
    mount_to_body(App);
}
