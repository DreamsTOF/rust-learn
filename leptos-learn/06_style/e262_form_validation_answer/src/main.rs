// ============================================================
// Exercise 262 - Answer
// ============================================================

use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    let email = RwSignal::new(String::new());
    let password = RwSignal::new(String::new());
    let errors = RwSignal::new(Vec::<String>::new());
    let submitted = RwSignal::new(false);

    let validate = move || {
        let mut errs = Vec::new();
        let e = email.get();
        let p = password.get();

        if e.is_empty() {
            errs.push("邮箱不能为空".to_string());
        } else if !e.contains('@') {
            errs.push("邮箱格式不正确（需要包含 @）".to_string());
        }

        if p.is_empty() {
            errs.push("密码不能为空".to_string());
        } else if p.len() < 6 {
            errs.push("密码长度不足，至少需要 6 个字符".to_string());
        }

        errors.set(errs);
        errors.with(|e| e.is_empty())
    };

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        if validate() {
            submitted.set(true);
        }
    };

    view! {
        <div>
            <h2>"表单验证示例"</h2>
            <form on:submit=on_submit>
                <div>
                    <label>"邮箱："</label>
                    <input
                        type="text"
                        prop:value=email
                        on:input=move |ev| email.set(event_target_value(&ev))
                    />
                </div>
                <div>
                    <label>"密码："</label>
                    <input
                        type="password"
                        prop:value=password
                        on:input=move |ev| password.set(event_target_value(&ev))
                    />
                </div>
                <button type="submit">"提交"</button>
            </form>
            {move || {
                let errs = errors.get();
                if !errs.is_empty() {
                    view! {
                        <ul style="color: red">
                            {errs.into_iter().map(|e| view! { <li>{e}</li> }).collect::<Vec<_>>()}
                        </ul>
                    }.into_any()
                } else if submitted.get() {
                    view! { <p style="color: green">"验证通过！"</p> }.into_any()
                } else {
                    view! { }.into_any()
                }
            }}
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
