// ============================================================
// 练习 e279: Tailwind CSS 集成 — 在 view! 中使用 Tailwind 类名
//
// 核心知识点:
//   - 在 Leptos 的 class 属性中使用 Tailwind CSS 类名
//   - Tailwind 响应式和交互式前缀（hover:, md: 等）
//   - Trunk + Tailwind 集成基础
//
// 难度: ⭐⭐ (补全关键代码)
// ============================================================

use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    view! {
        // TODO: 添加 Tailwind 类名: p-8 max-w-md mx-auto
        <div>
            // TODO: 添加 Tailwind 类名: text-2xl font-bold text-center text-blue-600 mb-4
            <h1>"Tailwind CSS 集成"</h1>

            // TODO: 添加 Tailwind 类名: bg-white shadow-md rounded-lg p-6
            <div>
                // TODO: 添加 Tailwind 类名: text-gray-700 mb-4
                <p>"这个组件的样式来自 Tailwind CSS。"</p>
                // TODO: 添加 Tailwind 类名: bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded
                <button>"按钮"</button>
            </div>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
