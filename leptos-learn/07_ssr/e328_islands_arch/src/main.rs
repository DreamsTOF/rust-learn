// ============================================================
// 练习 e328: Islands Architecture
//
// 核心知识点:
//   - Islands 模式：静态 HTML + 交互岛屿
//   - 交互式岛屿组件
//   - 懒加载水合
//
// 难度: ⭐⭐⭐ (最小指引)
// ============================================================

use leptos::prelude::*;

// TODO: 定义一个岛屿组件 CounterIsland
// ⭐⭐⭐ 提示:
//   - 使用不带 #[component] 的普通 fn（作为岛屿）
//   - 内部使用 RwSignal 或 signal() 管理状态
//   - view! 中包含按钮和计数显示

// TODO: 定义 Exercise 组件，包含标题说明 + 岛屿组件

fn main() {
    mount_to_body(|| view! { <p>"练习 328 (islands_arch)"</p> });
}
