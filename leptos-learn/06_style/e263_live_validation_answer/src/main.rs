// ============================================================
// Exercise 263 - Answer
// ============================================================

use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    let username = RwSignal::new(String::new());
    let email = RwSignal::new(String::new());

    let username_error = move || {
        let v = username.get();
        if v.is_empty() {
            Some("用户名不能为空".to_string())
        } else if v.len() < 3 {
            Some("用户名至少需要 3 个字符".to_string())
        } else {
            None
        }
    };

    let email_error = move || {
        let v = email.get();
        if v.is_empty() {
            Some("邮箱不能为空".to_string())
        } else if !v.contains('@') {
            Some("邮箱格式不正确".to_string())
        } else {
            None
        }
    };

    let is_valid = move || username_error().is_none() && email_error().is_none();

    view! {
        <div>
            <h2>"实时验证示例"</h2>
            <div>
                <label>"用户名："</label>
                <input
                    type="text"
                    prop:value=username
                    on:input=move |ev| username.set(event_target_value(&ev))
                />
                {move || username_error().map(|e| view! {
                    <span style="color: red; margin-left: 8px">{e}</span>
                })}
            </div>
            <div>
                <label>"邮箱："</label>
                <input
                    type="text"
                    prop:value=email
                    on:input=move |ev| email.set(event_target_value(&ev))
                />
                {move || email_error().map(|e| view! {
                    <span style="color: red; margin-left: 8px">{e}</span>
                })}
            </div>
            <p>
                {move || if is_valid() {
                    view! { <span style="color: green">"所有字段验证通过！"</span> }.into_any()
                } else {
                    view! { <span style="color: gray">"请填写正确的信息"</span> }.into_any()
                }}
            </p>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
