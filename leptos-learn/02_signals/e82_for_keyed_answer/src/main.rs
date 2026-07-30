use leptos::prelude::*;

#[derive(Debug, Clone)]
struct Task {
    id: u32,
    title: &'static str,
}

#[component]
fn Exercise() -> impl IntoView {
    let (tasks, _set_tasks) = signal(vec![
        Task { id: 1, title: "学习 Leptos" },
        Task { id: 2, title: "写练习" },
        Task { id: 3, title: "构建项目" },
    ]);

    view! {
        <h3>"任务列表"</h3>
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
