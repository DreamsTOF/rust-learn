// ============================================================
// Exercise 186 - Action Basic
// ============================================================

use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    let action = Action::new(|input: &String| {
        let input = input.clone();
        async move {
            format!("你好，{}！这是 Action 的处理结果。", input)
        }
    });

    let (name, set_name) = signal(String::new());

    view! {
        <div>
            <p>"练习 186 — Action 基础 (action_basic)"</p>
            <input
                type="text"
                placeholder="输入你的名字"
                on:input=move |ev| set_name(event_target_value(&ev))
                prop:value=name
            />
            <button
                on:click=move |_| { action.dispatch(name.get()); }
                disabled=move || action.pending().get()
            >
                {move || if action.pending().get() { "处理中..." } else { "提交" }}
            </button>
            <div>
                {move || action.value().get().map(|v| view! { <p><strong>{v}</strong></p> })}
            </div>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
