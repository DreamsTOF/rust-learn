// ============================================================
// 练习 e295: 客户端调用服务端函数
//
// 核心知识点:
//   - Action::new() 封装 server fn 调用
//   - action.dispatch(input) 触发调用
//   - action.value() 获取结果信号
//   - 客户端-服务端交互流程
//
// 难度: ⭐ (填空题 — 每行都有 TODO 指引)
// ============================================================

use leptos::prelude::*;
use leptos::prelude::ServerFnError;

// TODO: 定义 get_greeting server fn
// 提示: #[server(GetGreeting)]
// 提示: pub async fn get_greeting(name: String) -> Result<String, ServerFnError>
#[server(GetGreeting)]
pub async fn get_greeting(name: String) -> Result<String, ServerFnError> {
    // TODO: 返回 Ok(format!("你好，{}！来自服务器的问候。", name))
    Ok(format!("你好，{}！来自服务器的问候。", name))
}

#[component]
fn Exercise() -> impl IntoView {
    // TODO: 创建 Action 来调用 get_greeting server fn
    // 提示: Action::new(|input: &String| { ... get_greeting(input.clone()).await ... })
    let greet_action = Action::new(|input: &String| {
        let input = input.clone();
        async move {
            // TODO: 调用 get_greeting server fn 并返回结果
            // 提示: get_greeting(input.clone()).await
            get_greeting(input.clone()).await.unwrap_or_else(|e| e.to_string())
        }
    });

    let (name, set_name) = signal(String::new());

    view! {
        <div>
            <h2>"客户端调用服务端函数"</h2>
            // TODO: 添加说明段落
            <p>"使用 Action 封装 server fn 调用，自动管理加载和结果状态。"</p>

            // TODO: 添加输入框绑定 name 信号
            <input
                type="text"
                placeholder="输入你的名字"
                on:input=move |ev| set_name(event_target_value(&ev))
                prop:value=name
            />

            // TODO: 添加提交按钮
            <button
                on:click=move |_| { greet_action.dispatch(name.get()); }
                // TODO: 设置 disabled 属性：处理中时禁用
                disabled=move || greet_action.pending().get()
            >
                // TODO: 按钮文字：pending 时显示 "处理中..."，否则显示 "提交"
                {move || if greet_action.pending().get() { "处理中..." } else { "提交" }}
            </button>

            // TODO: 使用 action.value() 显示结果
            <div>
                {move || greet_action.value().get().map(|v| view! { <p><strong>{v}</strong></p> })}
            </div>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}

// <details>
// 参考答案:
//
// use leptos::prelude::*;
// use leptos::prelude::ServerFnError;
//
// #[server(GetGreeting)]
// pub async fn get_greeting(name: String) -> Result<String, ServerFnError> {
//     Ok(format!("你好，{}！来自服务器的问候。", name))
// }
//
// #[component]
// fn Exercise() -> impl IntoView {
//     let greet_action = Action::new(|input: &String| {
//         let input = input.clone();
//         async move {
//             get_greeting(input.clone()).await.unwrap_or_else(|e| e.to_string())
//         }
//     });
//
//     let (name, set_name) = signal(String::new());
//
//     view! {
//         <div>
//             <h2>"客户端调用服务端函数"</h2>
//             <p>"使用 Action 封装 server fn 调用，自动管理加载和结果状态。"</p>
//             <input
//                 type="text"
//                 placeholder="输入你的名字"
//                 on:input=move |ev| set_name(event_target_value(&ev))
//                 prop:value=name
//             />
//             <button
//                 on:click=move |_| { greet_action.dispatch(name.get()); }
//                 disabled=move || greet_action.pending().get()
//             >
//                 {move || if greet_action.pending().get() { "处理中..." } else { "提交" }}
//             </button>
//             <div>
//                 {move || greet_action.value().get().map(|v| view! { <p><strong>{v}</strong></p> })}
//             </div>
//         </div>
//     }
// }
//
// fn main() {
//     mount_to_body(Exercise);
// }
// </details>
