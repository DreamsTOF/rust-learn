// ============================================================
// 练习 e82: For Keyed — 用 key 闭包提供唯一标识
//
// 核心知识点:
//   - 为复杂数据类型提供 key 闭包
//   - 稳定的 key（如 id）能优化 DOM 差异对比性能
//
// 难度: ⭐⭐ (TODO 约 50%)
// ============================================================

use leptos::prelude::*;

#[derive(Debug, Clone)]
struct Task {
    id: u32,
    title: &'static str,
}

#[component]
fn Exercise() -> impl IntoView {
    // TODO: 创建信号 tasks 存储 Vec<Task>
    let (tasks, set_tasks) = signal(vec![
        Task { id: 1, title: "学习 Leptos" },
        Task { id: 2, title: "写练习" },
        Task { id: 3, title: "构建项目" },
    ]);

    view! {
        <h3>"任务列表"</h3>
        // TODO: 用 task.id 作为 key 渲染列表
        <For each=move || tasks.get() key=|task| task.id let:task>
            <p style="margin: 4px 0;">
                "☐ [" {task.id} "] " {task.title}
            </p>
        </For>
    }
}

fn main() {
    mount_to_body(Exercise);
}

// <details>
// 参考答案（去除注释后的纯净版本）:
//
// use leptos::prelude::*;
//
// #[derive(Debug, Clone)]
// struct Task {
//     id: u32,
//     title: &'static str,
// }
//
// #[component]
// fn Exercise() -> impl IntoView {
//     let (tasks, set_tasks) = signal(vec![
//         Task { id: 1, title: "学习 Leptos" },
//         Task { id: 2, title: "写练习" },
//         Task { id: 3, title: "构建项目" },
//     ]);
//
//     view! {
//         <h3>"任务列表"</h3>
//         <For each=move || tasks.get() key=|task| task.id let:task>
//             <p style="margin: 4px 0;">
//                 "☐ [" {task.id} "] " {task.title}
//             </p>
//         </For>
//     }
// }
//
// fn main() {
//     mount_to_body(Exercise);
// }
//
// ### 知识点
// - 对于复杂结构，key 应使用稳定且唯一的字段（如自增 id）
// - 好的 key 帮助 Leptos 精准定位变化元素，减少 DOM 操作
// - key 的类型需实现 Eq + Hash（如 u32、String 等）
// </details>
