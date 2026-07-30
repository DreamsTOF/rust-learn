// ============================================================
// Exercise 298 - Answer
// ============================================================

use leptos::prelude::*;
use leptos::prelude::ServerFnError;

#[server(ValidateUser)]
pub async fn validate_user(name: String, age: i32) -> Result<String, ServerFnError> {
    if name.trim().is_empty() {
        return Err(ServerFnError::ServerError("姓名不能为空".into()));
    }
    if age < 18 {
        return Err(ServerFnError::ServerError("年龄必须大于等于 18 岁".into()));
    }
    Ok(format!("验证通过！欢迎，{}！", name))
}

#[component]
fn Exercise() -> impl IntoView {
    let validate_action = Action::new(|input: &ValidateUser| {
        let input = input.clone();
        async move {
            validate_user(input.name.clone(), input.age).await
        }
    });

    let (name, set_name) = signal(String::new());
    let (age, set_age) = signal(String::new());

    view! {
        <div style="padding: 1.5rem; font-family: system-ui, sans-serif; max-width: 32rem; margin: 0 auto;">
            <h2 style="border-bottom: 2px solid #e2e8f0; padding-bottom: 0.5rem;">
                "练习 298: 服务端验证"
            </h2>
            <p style="color: #475569;">"提交后由服务端验证输入合法性。"</p>

            <div style="margin: 0.75rem 0;">
                <input
                    type="text"
                    placeholder="输入姓名"
                    on:input=move |ev| set_name(event_target_value(&ev))
                    prop:value=name
                    style="padding: 0.5rem; width: 100%; box-sizing: border-box; border: 1px solid #cbd5e1; border-radius: 4px;"
                />
            </div>
            <div style="margin: 0.75rem 0;">
                <input
                    type="number"
                    placeholder="输入年龄"
                    on:input=move |ev| set_age(event_target_value(&ev))
                    prop:value=age
                    style="padding: 0.5rem; width: 100%; box-sizing: border-box; border: 1px solid #cbd5e1; border-radius: 4px;"
                />
            </div>
            <div style="margin: 0.75rem 0;">
                <button
                    on:click=move |_| {
                        let age_val = age.get().parse::<i32>().unwrap_or(0);
                        validate_action.dispatch(ValidateUser { name: name.get(), age: age_val });
                    }
                    disabled=move || validate_action.pending().get()
                    style="padding: 0.5rem 1rem; cursor: pointer; background: #3b82f6; color: white; border: none; border-radius: 4px;"
                >
                    {move || if validate_action.pending().get() { "验证中..." } else { "提交验证" }}
                </button>
            </div>

            <div style="margin: 0.75rem 0;">
                {move || validate_action.value().get().map(|result| match result {
                    Ok(msg) => view! { <p style="color: #16a34a; font-weight: bold;">{msg}</p> }.into_any(),
                    Err(e) => view! { <p style="color: #dc2626; font-weight: bold;">{format!("验证失败：{}", e.to_string())}</p> }.into_any(),
                })}
            </div>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
