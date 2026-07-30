// ============================================================
// Exercise 188 - Action Pending
// ============================================================

use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    let (input, set_input) = signal(String::new());

    let action = Action::new(|input: &String| {
        let input = input.clone();
        async move {
            format!("处理完成！输入内容长度: {} 字符", input.len())
        }
    });

    view! {
        <div>
            <p>"练习 188 — Action 加载状态 (action_pending)"</p>
            <input type="text" placeholder="输入一些文字"
                on:input=move |ev| set_input(event_target_value(&ev))
                prop:value=move || input.get()
            />
            <button
                on:click=move |_| { action.dispatch(input.get()); }
                disabled=move || action.pending().get()
            >
                {move || if action.pending().get() { "处理中..." } else { "开始处理" }}
            </button>
            <div>
                {move || if action.pending().get() {
                    view! { <p style="color: orange;">"⏳ 正在处理，请勿重复提交..."</p> }.into_any()
                } else {
                    view! {}.into_any()
                }}
                {move || action.value().get().map(|v| view! { <p style="color: green;"><strong>{v}</strong></p> })}
            </div>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
