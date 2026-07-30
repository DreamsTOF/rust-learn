// ============================================================
// Exercise 295 - Answer
// ============================================================

use leptos::prelude::*;
use leptos::prelude::ServerFnError;

#[server(GetGreeting)]
pub async fn get_greeting(name: String) -> Result<String, ServerFnError> {
    Ok(format!("你好，{}！来自服务器的问候。", name))
}

#[component]
fn Exercise() -> impl IntoView {
    let greet_action = Action::new(|input: &String| {
        let input = input.clone();
        async move {
            get_greeting(input.clone()).await.unwrap_or_else(|e| e.to_string())
        }
    });

    let (name, set_name) = signal(String::new());

    view! {
        <div>
            <h2>"客户端调用服务端函数"</h2>
            <p>"使用 Action 封装 server fn 调用，自动管理加载和结果状态。"</p>
            <input
                type="text"
                placeholder="输入你的名字"
                on:input=move |ev| set_name(event_target_value(&ev))
                prop:value=name
            />
            <button
                on:click=move |_| { greet_action.dispatch(name.get()); }
                disabled=move || greet_action.pending().get()
            >
                {move || if greet_action.pending().get() { "处理中..." } else { "提交" }}
            </button>
            <div>
                {move || greet_action.value().get().map(|v| view! { <p><strong>{v}</strong></p> })}
            </div>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
