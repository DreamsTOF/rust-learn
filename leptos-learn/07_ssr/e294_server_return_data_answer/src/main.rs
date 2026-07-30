// ============================================================
// Exercise 294 - Answer
// ============================================================

use leptos::prelude::*;
use leptos::prelude::ServerFnError;

#[server(GetUserInfo)]
pub async fn get_user_info() -> Result<String, ServerFnError> {
    Ok("用户: admin，角色: 管理员".to_string())
}

#[server(GetServerTime)]
pub async fn get_server_time() -> Result<String, ServerFnError> {
    Ok("服务器时间: 2025-01-01 12:00:00".to_string())
}

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <div>
            <h2>"服务端函数 — 返回值类型"</h2>
            <p>"server fn 必须返回 Result<T, ServerFnError>"</p>
            <ul>
                <li>"成功: Ok(T) — T 可以是 String、Vec、struct 等"</li>
                <li>"错误: Err(ServerFnError::new(...)) — 创建自定义错误"</li>
                <li>"ServerFnError 来自 " <code>"leptos::server::ServerFnError"</code></li>
            </ul>
            <p>"server fn 返回值自动序列化（通过 serde），客户端反序列化后使用。"</p>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
