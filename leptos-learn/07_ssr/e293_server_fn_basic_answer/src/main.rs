// ============================================================
// Exercise 293 - Answer
// ============================================================

use leptos::prelude::*;
use leptos::prelude::ServerFnError;

#[server(HelloServer)]
pub async fn hello_server() -> Result<String, ServerFnError> {
    Ok("你好，服务器！".to_string())
}

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <div>
            <h2>"#[server] 宏 — 基础语法"</h2>
            <p>"server fn 使用 #[server] 属性宏标记"</p>
            <ul>
                <li>"宏参数: #[server(ServerFnName)]"</li>
                <li>"返回类型: " "Result<T, ServerFnError>""</li>
                <li>"作用: " "在客户端调用的服务端函数""</li>
            </ul>
            <p>"server fn 中的代码在服务端执行，客户端通过 HTTP 调用。"</p>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
