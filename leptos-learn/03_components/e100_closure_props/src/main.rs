// ============================================================
// 练习 e100: 闭包 Props (closure_props)
//
// 核心知识点:
//   - 使用 impl Fn() + 'static 作为事件回调
//   - 在子组件内部将闭包绑定到 DOM 事件
//
// 难度: ⭐⭐
// ============================================================

use leptos::prelude::*;

// TODO: 定义 Button 组件，接收 on_click: impl Fn() + 'static 闭包 prop
// 在按钮点击时调用 on_click
#[component]
fn Button(on_click: impl Fn() + 'static) -> impl IntoView {
    view! {
        // TODO: 将 on_click 绑定到按钮的 on:click 事件
        <button on:click=move |_| on_click()>"Click me"</button>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <div>
            // TODO: 传入闭包，点击时输出日志
            // 提示: 使用 leptos::logging::log! 宏
            <Button on_click=|| leptos::logging::log!("Button clicked!") />
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}

// ============================================================
// 参考答案 (思考后再看!)
// ============================================================
// <details>
// <summary>点击展开答案</summary>
//
// ### 代码
// ```rust
// use leptos::prelude::*;
//
// #[component]
// fn Button(on_click: impl Fn() + 'static) -> impl IntoView {
//     view! {
//         <button on:click=move |_| on_click()>"Click me"</button>
//     }
// }
//
// #[component]
// fn Exercise() -> impl IntoView {
//     view! {
//         <div>
//             <Button on_click=|| leptos::logging::log!("Button clicked!") />
//         </div>
//     }
// }
//
// fn main() {
//     mount_to_body(Exercise);
// }
// ```
//
// ### 知识点
// - impl Fn() + 'static 允许传入任何可调用的闭包或函数
// - 组件内部将闭包包装为事件处理器：move |ev| on_click()
// - 'static 生命周期确保闭包在组件整个生命周期内有效
// - 闭包 Props 是 Leptos 组件实现回调的常用模式
//
// </details>
