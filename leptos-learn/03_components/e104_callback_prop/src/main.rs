// ============================================================
// 练习 e104: Callback Prop — 通过回调 prop 实现子传父通信
//
// 核心知识点:
//   - 泛型回调 prop（父传闭包，子组件调用）
//   - 子组件通过闭包参数向父组件发送消息
//
// 难度: ⭐⭐
// ============================================================

use leptos::prelude::*;

// ConfirmButton 接收泛型回调 on_confirm
// 点击按钮时调用 on_confirm() 通知父组件
//
// 使用泛型 F 而非 Box<dyn Fn()>，避免动态分发开销
// 约束: F: Fn() + 'static 即可以在任意线程中安全调用
#[component]
fn ConfirmButton<F>(on_confirm: F) -> impl IntoView
where
    F: Fn() + 'static,
{
    view! {
        <button
            style="background:#e74c3c;color:white;border:none;padding:8px 16px;border-radius:4px;cursor:pointer;"
            on:click=move |_| on_confirm()
        >
            "确认删除"
        </button>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    let (msg, set_msg) = signal("等待操作...".to_string());
    view! {
        <p>{msg}</p>
        // TODO: 传入 on_confirm 回调，点击时更新消息
        <ConfirmButton on_confirm=move || set_msg.set("已确认删除！".to_string()) />
    }
}

fn main() {
    mount_to_body(Exercise);
}

// <details>
// 参考答案（去除注释后的纯净版本）:
//
// use leptos::prelude::*;
//
// #[component]
// fn ConfirmButton<F>(on_confirm: F) -> impl IntoView
// where
//     F: Fn() + 'static,
// {
//     view! {
//         <button
//             style="background:#e74c3c;color:white;border:none;padding:8px 16px;border-radius:4px;cursor:pointer;"
//             on:click=move |_| on_confirm()
//         >
//             "确认删除"
//         </button>
//     }
// }
//
// #[component]
// fn Exercise() -> impl IntoView {
//     let (msg, set_msg) = signal("等待操作...".to_string());
//     view! {
//         <p>{msg}</p>
//         <ConfirmButton on_confirm=move || set_msg.set("已确认删除！".to_string()) />
//     }
// }
//
// fn main() {
//     mount_to_body(Exercise);
// }
// </details>
