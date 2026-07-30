// ============================================================
// 练习 e279: Tailwind CSS 集成 — 参考答案
//
// 核心知识点:
//   - 在 Leptos 的 class 属性中使用 Tailwind CSS 类名
//   - Tailwind 响应式和交互式前缀（hover:, md: 等）
//   - Trunk + Tailwind 集成基础
// ============================================================

use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <div class="p-8 max-w-md mx-auto">
            <h1 class="text-2xl font-bold text-center text-blue-600 mb-4">
                "Tailwind CSS 集成"
            </h1>
            <div class="bg-white shadow-md rounded-lg p-6">
                <p class="text-gray-700 mb-4">
                    "这个组件的样式来自 Tailwind CSS。"
                </p>
                <button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded">
                    "按钮"
                </button>
            </div>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
