// ============================================================
// 练习 e294: 服务端函数返回值
//
// 核心知识点:
//   - Result<T, ServerFnError> 作为返回值
//   - ServerFnError::new() 创建服务端错误
//   - 结构化数据返回
//   - 演练三种返回模式
//
// 难度: ⭐ (填空题 — 每行都有 TODO 指引)
// ============================================================

use leptos::prelude::*;
use leptos::prelude::ServerFnError;

// TODO: 定义 getUserInfo server fn
// 提示: #[server(GetUserInfo)]
// 提示: 返回 Result<String, ServerFnError>
#[server(GetUserInfo)]
pub async fn get_user_info() -> Result<String, ServerFnError> {
    // TODO: 成功时返回 Ok("用户: admin，角色: 管理员")
    Ok("用户: admin，角色: 管理员".to_string())
}

// TODO: 定义 getServerTime server fn
// 提示: 返回 Result<String, ServerFnError>
#[server(GetServerTime)]
pub async fn get_server_time() -> Result<String, ServerFnError> {
    // TODO: 返回 Ok("服务器时间: 2025-01-01 12:00:00")
    Ok("服务器时间: 2025-01-01 12:00:00".to_string())
}

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <div>
            <h2>"服务端函数 — 返回值类型"</h2>
            <p>"server fn 必须返回 Result<T, ServerFnError>"</p>
            // TODO: 添加 <ul> 列举返回值知识
            <ul>
                // TODO: 补全列表项
                <li>"成功: Ok(T) — T 可以是 String、Vec、struct 等"</li>
                <li>"错误: Err(ServerFnError::new(...)) — 创建自定义错误"</li>
                <li>"ServerFnError 来自 " <code>"leptos::server::ServerFnError"</code></li>
            </ul>
            // TODO: 添加说明：服务端返回的数据会序列化后传给客户端
            <p>"server fn 返回值自动序列化（通过 serde），客户端反序列化后使用。"</p>
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
// #[server(GetUserInfo)]
// pub async fn get_user_info() -> Result<String, ServerFnError> {
//     Ok("用户: admin，角色: 管理员".to_string())
// }
//
// #[server(GetServerTime)]
// pub async fn get_server_time() -> Result<String, ServerFnError> {
//     Ok("服务器时间: 2025-01-01 12:00:00".to_string())
// }
//
// #[component]
// fn Exercise() -> impl IntoView {
//     view! {
//         <div>
//             <h2>"服务端函数 — 返回值类型"</h2>
//             <p>"server fn 必须返回 Result<T, ServerFnError>"</p>
//             <ul>
//                 <li>"成功: Ok(T) — T 可以是 String、Vec、struct 等"</li>
//                 <li>"错误: Err(ServerFnError::new(...)) — 创建自定义错误"</li>
//                 <li>"ServerFnError 来自 " <code>"leptos::server::ServerFnError"</code></li>
//             </ul>
//             <p>"server fn 返回值自动序列化（通过 serde），客户端反序列化后使用。"</p>
//         </div>
//     }
// }
//
// fn main() {
//     mount_to_body(Exercise);
// }
// </details>
