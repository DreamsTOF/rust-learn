// ============================================================
// 练习 e298: 服务端验证 (Server Validation)
//
// 核心知识点:
//   - #[server] 函数中进行输入验证
//   - 验证失败返回 ServerFnError
//   - 客户端显示验证错误消息
//   - 结合 Action 和服务端函数实现表单验证
//
// 难度: ⭐⭐ (关键 TODO 已标记)
// ============================================================

use leptos::prelude::*;
use leptos::prelude::ServerFnError;

// ⭐⭐ TODO: 使用 #[server(ValidateUser)] 定义 validate_user 服务端函数
// 接受 name: String 和 age: i32
// 验证 name 非空，age >= 18
// 返回 Result<String, ServerFnError>
// 提示: 验证失败用 Err(ServerFnError::ServerError("消息".into()))

#[component]
fn Exercise() -> impl IntoView {
    // ⭐⭐ TODO: 创建 Action 包装 validate_user
    // 提示: Action::new(|input: &ValidateUser| { async move { ... } })

    let (name, set_name) = signal(String::new());
    let (age, set_age) = signal(String::new());

    view! {
        <div>
            <h2>"练习 298: 服务端验证"</h2>
            <p>"提交后由服务端验证输入合法性。"</p>

            <div>
                <input
                    type="text"
                    placeholder="输入姓名"
                    on:input=move |ev| set_name(event_target_value(&ev))
                    prop:value=name
                />
            </div>
            <div>
                <input
                    type="number"
                    placeholder="输入年龄"
                    on:input=move |ev| set_age(event_target_value(&ev))
                    prop:value=age
                />
            </div>
            <div>
                <button
                    on:click=move |_| {
                        let age_val = age.get().parse::<i32>().unwrap_or(0);
                        validate_action.dispatch(ValidateUser { name: name.get(), age: age_val });
                    }
                    disabled=move || validate_action.pending().get()
                >
                    {move || if validate_action.pending().get() { "验证中..." } else { "提交验证" }}
                </button>
            </div>

            // ⭐⭐ TODO: 使用 action.value() 显示验证结果
            // 提示: match result { Ok(msg) => 绿色, Err(e) => 红色 }
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
// #[server(ValidateUser)]
// pub async fn validate_user(name: String, age: i32) -> Result<String, ServerFnError> {
//     if name.trim().is_empty() {
//         return Err(ServerFnError::ServerError("姓名不能为空".into()));
//     }
//     if age < 18 {
//         return Err(ServerFnError::ServerError("年龄必须大于等于 18 岁".into()));
//     }
//     Ok(format!("验证通过！欢迎，{}！", name))
// }
//
// #[component]
// fn Exercise() -> impl IntoView {
//     let validate_action = Action::new(|input: &ValidateUser| {
//         let input = input.clone();
//         async move {
//             validate_user(input.name.clone(), input.age).await
//         }
//     });
//
//     let (name, set_name) = signal(String::new());
//     let (age, set_age) = signal(String::new());
//
//     view! {
//         <div>
//             <h2>"练习 298: 服务端验证"</h2>
//             <p>"提交后由服务端验证输入合法性。"</p>
//
//             <div>
//                 <input type="text" placeholder="输入姓名"
//                     on:input=move |ev| set_name(event_target_value(&ev))
//                     prop:value=name />
//             </div>
//             <div>
//                 <input type="number" placeholder="输入年龄"
//                     on:input=move |ev| set_age(event_target_value(&ev))
//                     prop:value=age />
//             </div>
//             <div>
//                 <button
//                     on:click=move |_| {
//                         let age_val = age.get().parse::<i32>().unwrap_or(0);
//                         validate_action.dispatch(ValidateUser { name: name.get(), age: age_val });
//                     }
//                     disabled=move || validate_action.pending().get()
//                 >
//                     {move || if validate_action.pending().get() { "验证中..." } else { "提交验证" }}
//                 </button>
//             </div>
//             <div>
//                 {move || validate_action.value().get().map(|result| match result {
//                     Ok(msg) => view! { <p style="color: green">{msg}</p> }.into_any(),
//                     Err(e) => view! { <p style="color: red">{format!("验证失败：{}", e.to_string())}</p> }.into_any(),
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
