// ============================================================
// 练习 e293: #[server] 基础语法
//
// 核心知识点:
//   - #[server] 宏: 将函数标记为服务端可调用函数
//   - #[server(MyFn)] 语法: 为 server fn 命名
//   - ServerFnError: 服务端函数的错误类型
//   - 编译为 CSR 练习，这里展示语法定义
//
// 难度: ⭐ (填空题 — 每行都有 TODO 指引)
// ============================================================

use leptos::prelude::*;
use leptos::prelude::ServerFnError;

// TODO: 使用 #[server] 宏标记此函数为服务端函数
// 提示: #[server(函数别名)]
// 提示: pub async fn 函数名() -> Result<返回类型, ServerFnError>
#[server(HelloServer)]
pub async fn hello_server() -> Result<String, ServerFnError> {
    // TODO: 返回 "你好，服务器！" 用 Ok 包裹
    Ok("你好，服务器！".to_string())
}

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <div>
            <h2>"#[server] 宏 — 基础语法"</h2>
            <p>"server fn 使用 #[server] 属性宏标记"</p>
            <ul>
                // TODO: 补全列表项描述 server fn 的三个要素
                <li>"宏参数: #[server(ServerFnName)]"</li>
                <li>"返回类型: " "Result<T, ServerFnError>""</li>
                <li>"作用: " "在客户端调用的服务端函数""</li>
            </ul>
            // TODO: 添加说明：server fn 的代码运行在服务端
            <p>"server fn 中的代码在服务端执行，客户端通过 HTTP 调用。"</p>
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
// #[server(HelloServer)]
// pub async fn hello_server() -> Result<String, ServerFnError> {
//     Ok("你好，服务器！".to_string())
// }
//
// #[component]
// fn Exercise() -> impl IntoView {
//     view! {
//         <div>
//             <h2>"#[server] 宏 — 基础语法"</h2>
//             <p>"server fn 使用 #[server] 属性宏标记"</p>
//             <ul>
//                 <li>"宏参数: #[server(ServerFnName)]"</li>
//                 <li>"返回类型: " "Result<T, ServerFnError>""</li>
//                 <li>"作用: " "在客户端调用的服务端函数""</li>
//             </ul>
//             <p>"server fn 中的代码在服务端执行，客户端通过 HTTP 调用。"</p>
//         </div>
//     }
// }
//
// fn main() {
//     mount_to_body(Exercise);
// }
// </details>
