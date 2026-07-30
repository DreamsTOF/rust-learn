// ============================================================
// Exercise 300 - Answer
// ============================================================

use leptos::prelude::*;
use leptos::prelude::ServerFnError;

#[server(AddTask)]
pub async fn add_task(name: String, description: String) -> Result<String, ServerFnError> {
    if name.trim().is_empty() {
        return Err(ServerFnError::ServerError("任务名称不能为空".into()));
    }
    // 真实 SSR 应用中会使用 sqlx 执行参数化 INSERT:
    //   sqlx::query("INSERT INTO tasks (name, description) VALUES ($1, $2)")
    //       .bind(&name).bind(&description)
    //       .execute(pool).await
    //       .map_err(|e| ServerFnError::ServerError(e.to_string()))?;
    //
    Ok(format!("任务「{}」已成功添加！", name))
}

#[component]
fn Exercise() -> impl IntoView {
    let add_action = Action::new(|input: &AddTask| {
        let input = input.clone();
        async move {
            add_task(input.name.clone(), input.description.clone()).await
        }
    });

    let (name, set_name) = signal(String::new());
    let (description, set_description) = signal(String::new());

    view! {
        <div style="padding: 1.5rem; font-family: system-ui, sans-serif; max-width: 32rem; margin: 0 auto;">
            <h2 style="border-bottom: 2px solid #e2e8f0; padding-bottom: 0.5rem;">
                "练习 300: 数据库写入"
            </h2>
            <p style="color: #475569;">"添加新任务到数据库。"</p>

            <div style="margin: 0.75rem 0;">
                <input
                    type="text"
                    placeholder="任务名称"
                    on:input=move |ev| set_name(event_target_value(&ev))
                    prop:value=name
                    style="padding: 0.5rem; width: 100%; box-sizing: border-box; border: 1px solid #cbd5e1; border-radius: 4px;"
                />
            </div>
            <div style="margin: 0.75rem 0;">
                <input
                    type="text"
                    placeholder="任务描述"
                    on:input=move |ev| set_description(event_target_value(&ev))
                    prop:value=description
                    style="padding: 0.5rem; width: 100%; box-sizing: border-box; border: 1px solid #cbd5e1; border-radius: 4px;"
                />
            </div>
            <div style="margin: 0.75rem 0;">
                <button
                    on:click=move |_| {
                        add_action.dispatch(AddTask {
                            name: name.get(),
                            description: description.get(),
                        });
                        set_name(String::new());
                        set_description(String::new());
                    }
                    disabled=move || add_action.pending().get()
                    style="padding: 0.5rem 1rem; cursor: pointer; background: #3b82f6; color: white; border: none; border-radius: 4px;"
                >
                    {move || if add_action.pending().get() { "提交中..." } else { "添加任务" }}
                </button>
            </div>

            <div style="margin: 0.75rem 0;">
                {move || add_action.value().get().map(|result| match result {
                    Ok(msg) => view! { <p style="color: #16a34a; font-weight: bold;">{msg}</p> }.into_any(),
                    Err(e) => view! { <p style="color: #dc2626; font-weight: bold;">{format!("错误：{}", e.to_string())}</p> }.into_any(),
                })}
            </div>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
