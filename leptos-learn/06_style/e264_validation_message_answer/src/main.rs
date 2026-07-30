// ============================================================
// Exercise 264 - Answer
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

        // 邮箱验证：多条消息
        if e.is_empty() {
            errs.push("邮箱：必填".to_string());
        } else {
            if !e.contains('@') {
                errs.push("邮箱：格式错误（缺少 @）".to_string());
            }
            if !e.contains('.') {
                errs.push("邮箱：格式错误（缺少域名后缀）".to_string());
            }
        }

        // 密码验证：多条消息
        if p.is_empty() {
            errs.push("密码：必填".to_string());
        } else {
            if p.len() < 6 {
                errs.push("密码：长度不足（至少 6 位）".to_string());
            }
            if !p.chars().any(|c| c.is_ascii_digit()) {
                errs.push("密码：需包含数字".to_string());
            }
            if !p.chars().any(|c| c.is_ascii_uppercase()) {
                errs.push("密码：需包含大写字母".to_string());
            }
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
            <h2>"验证消息示例"</h2>
            <form on:submit=on_submit>
                <div>
                    <label>"邮箱："</label>
                    <input
                        type="text"
                        prop:value=email
                        on:input=move |ev| {
                            email.set(event_target_value(&ev));
                            submitted.set(false);
                        }
                    />
                </div>
                <div>
                    <label>"密码："</label>
                    <input
                        type="password"
                        prop:value=password
                        on:input=move |ev| {
                            password.set(event_target_value(&ev));
                            submitted.set(false);
                        }
                    />
                </div>
                <button type="submit">"提交"</button>
            </form>
            {move || {
                let errs = errors.get();
                if !errs.is_empty() {
                    view! {
                        <div style="color: red; margin-top: 8px">
                            <p>"请修正以下错误："</p>
                            <ul>
                                {errs.into_iter().map(|e| view! { <li>{e}</li> }).collect::<Vec<_>>()}
                            </ul>
                        </div>
                    }.into_any()
                } else if submitted.get() {
                    view! { <p style="color: green; margin-top: 8px">"验证通过！所有字段符合要求。"</p> }.into_any()
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
