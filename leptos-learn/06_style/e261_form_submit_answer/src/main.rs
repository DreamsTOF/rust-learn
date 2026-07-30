// ============================================================
// Exercise 261 - Answer
// ============================================================

use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    let name = RwSignal::new(String::new());
    let email = RwSignal::new(String::new());
    let submitted = RwSignal::new(None::<(String, String)>);

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        submitted.set(Some((name.get(), email.get())));
    };

    view! {
        <div>
            <h2>"表单提交示例"</h2>
            <form on:submit=on_submit>
                <div>
                    <label>"姓名："</label>
                    <input
                        type="text"
                        prop:value=name
                        on:input=move |ev| name.set(event_target_value(&ev))
                    />
                </div>
                <div>
                    <label>"邮箱："</label>
                    <input
                        type="email"
                        prop:value=email
                        on:input=move |ev| email.set(event_target_value(&ev))
                    />
                </div>
                <button type="submit">"提交"</button>
            </form>
            {move || submitted.get().map(|(n, e)| view! {
                <div style="margin-top: 12px; padding: 8px; border: 1px solid #ccc;">
                    <p>"提交成功！"</p>
                    <p>"姓名：" {n}</p>
                    <p>"邮箱：" {e}</p>
                </div>
            })}
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
