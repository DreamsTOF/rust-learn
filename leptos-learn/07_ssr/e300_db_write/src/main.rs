// ============================================================
// 练习 e300: 数据库写入 (DB Write)
//
// 核心知识点:
//   - #[server] 函数执行 INSERT / UPDATE 操作
//   - 参数化查询（防止 SQL 注入）
//   - 使用 Action 提交数据到服务端
//   - 显示操作结果（成功 / 失败）
//
// 难度: ⭐⭐ (关键 TODO 已标记)
// ============================================================

use leptos::prelude::*;
use leptos::prelude::ServerFnError;

// ⭐⭐ TODO: 使用 #[server(AddTask)] 定义 add_task 服务端函数
// 接受 name: String, description: String
// 返回 Result<String, ServerFnError>
// 提示: 验证输入非空，然后返回插入成功的确认消息
// 在真实应用中，函数体会使用 sqlx 执行:
//   sqlx::query("INSERT INTO tasks (name, description) VALUES ($1, $2)")
//       .bind(&name).bind(&description)
//       .execute(pool).await
//       .map_err(|e| ServerFnError::ServerError(e.to_string()))?;

#[component]
fn Exercise() -> impl IntoView {
    // ⭐⭐ TODO: 创建 Action 包装 add_task
    // 提示: Action::new(|input: &AddTask| { async move { ... } })

    let (name, set_name) = signal(String::new());
    let (description, set_description) = signal(String::new());

    view! {
        <div>
            <h2>"练习 300: 数据库写入"</h2>
            <p>"添加新任务到数据库。"</p>

            <div>
                <input
                    type="text"
                    placeholder="任务名称"
                    on:input=move |ev| set_name(event_target_value(&ev))
                    prop:value=name
                />
            </div>
            <div>
                <input
                    type="text"
                    placeholder="任务描述"
                    on:input=move |ev| set_description(event_target_value(&ev))
                    prop:value=description
                />
            </div>
            <div>
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
                >
                    {move || if add_action.pending().get() { "提交中..." } else { "添加任务" }}
                </button>
            </div>

            // ⭐⭐ TODO: 使用 action.value() 显示操作结果
            // 提示: match result { Ok(msg) => 绿色成功, Err(e) => 红色错误 }
            <div>
            </div>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}

// <details>
// <summary>点击展开答案</summary>
//
// ```rust
// use leptos::prelude::*;
// use leptos::prelude::ServerFnError;
//
// #[server(AddTask)]
// pub async fn add_task(name: String, description: String) -> Result<String, ServerFnError> {
//     if name.trim().is_empty() {
//         return Err(ServerFnError::ServerError("任务名称不能为空".into()));
//     }
//     // 真实应用中:
//     // sqlx::query("INSERT INTO tasks (name, description) VALUES ($1, $2)")
//     //     .bind(&name).bind(&description)
//     //     .execute(pool).await
//     //     .map_err(|e| ServerFnError::ServerError(e.to_string()))?;
//     Ok(format!("任务「{}」已成功添加！", name))
// }
//
// #[component]
// fn Exercise() -> impl IntoView {
//     let add_action = Action::new(|input: &AddTask| {
//         let input = input.clone();
//         async move {
//             add_task(input.name.clone(), input.description.clone()).await
//         }
//     });
//
//     let (name, set_name) = signal(String::new());
//     let (description, set_description) = signal(String::new());
//
//     view! {
//         <div style="padding: 1.5rem; font-family: system-ui, sans-serif; max-width: 32rem; margin: 0 auto;">
//             <h2>"练习 300: 数据库写入"</h2>
//             <p>"添加新任务到数据库。"</p>
//             <div><input type="text" placeholder="任务名称"
//                 on:input=move |ev| set_name(event_target_value(&ev))
//                 prop:value=name /></div>
//             <div><input type="text" placeholder="任务描述"
//                 on:input=move |ev| set_description(event_target_value(&ev))
//                 prop:value=description /></div>
//             <div>
//                 <button on:click=move |_| {
//                     add_action.dispatch(AddTask { name: name.get(), description: description.get() });
//                     set_name(String::new());
//                     set_description(String::new());
//                 } disabled=move || add_action.pending().get()>
//                     {move || if add_action.pending().get() { "提交中..." } else { "添加任务" }}
//                 </button>
//             </div>
//             <div>
//                 {move || add_action.value().get().map(|result| match result {
//                     Ok(msg) => view! { <p style="color: green">{msg}</p> }.into_any(),
//                     Err(e) => view! { <p style="color: red">{format!("错误：{}", e.to_string())}</p> }.into_any(),
//                 })}
//             </div>
//         </div>
//     }
// }
//
// fn main() {
//     mount_to_body(Exercise);
// }
// ```
// </details>
