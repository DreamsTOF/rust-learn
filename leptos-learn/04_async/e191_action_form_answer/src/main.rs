// ============================================================
// Exercise 191 - Action Form
// ============================================================

use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    let (name, set_name) = signal(String::new());
    let action = Action::new(|input: &String| {
        let input = input.clone();
        async move {
            format!("你好，{}！消息已收到：处理成功", input)
        }
    });

    view! {
        <div>
            <p>"练习 191 — Action 与表单 (action_form)"</p>
            <form on:submit=move |ev| {
                ev.prevent_default();
                action.dispatch(name.get());
            }>
                <div>
                    <label>"姓名: "
                        <input type="text"
                            prop:value=move || name.get()
                            on:input=move |ev| set_name(event_target_value(&ev))
                        />
                    </label>
                </div>
                <button type="submit" disabled=move || action.pending().get()>
                    {move || if action.pending().get() { "提交中..." } else { "提交" }}
                </button>
            </form>
            <hr />
            <div>
                {move || match action.value().get() {
                    None => view! { <p>"尚未提交"</p> }.into_any(),
                    Some(result) => view! { <pre>{result}</pre> }.into_any(),
                }}
            </div>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
