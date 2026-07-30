// ============================================================
// 练习 e329: Code Splitting & SSR
//
// 核心知识点:
//   - 动态导入 (dynamic import)
//   - 懒加载组件
//   - 代码分块 (chunk splitting)
//
// 难度: ⭐⭐⭐ (最小指引)
// ============================================================

use leptos::prelude::*;

// TODO: 定义一个懒加载的 HeavierComponent
// ⭐⭐⭐ 提示:
//   - 使用 leptos::future::spawn_local 模拟异步加载
//   - RwSignal<Option< impl IntoView >> 管理状态
//   - 加载时显示 "加载中..."，完成后显示内容

// TODO: 定义 Exercise 组件，包含懒加载区域

fn main() {
    mount_to_body(|| view! { <p>"练习 329 (code_split_ssr)"</p> });
}
