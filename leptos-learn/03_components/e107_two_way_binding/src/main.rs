// ============================================================
// 练习 e107: 双向绑定 (Two-Way Binding)
//
// 核心知识点:
//   - value + on_change 模式模拟双向绑定
//   - prop:value 设置 DOM 属性实现受控输入
//   - 泛型闭包 F: Fn(String) 作为回调 prop
//
// 难度: ⭐⭐⭐
// ============================================================

use leptos::prelude::*;

// 受控输入组件：接收 value 和 on_change，实现双向绑定
#[component]
fn BindedInput<F>(value: String, on_change: F) -> impl IntoView
where
    F: Fn(String) + 'static,
{
    view! {
        <input
            type="text"
            // TODO: 使用 prop:value 绑定 value（设置 DOM 属性而非 HTML 属性）
            prop:value={value}
            // TODO: 在 on:input 中通知父组件值已变化
            on:input=move |ev| {
                on_change(event_target_value(&ev));
            }
        />
    }
}

#[component]
fn App() -> impl IntoView {
    let (name, set_name) = signal(String::from("Leptos"));

    view! {
        <h3>"练习 107: two_way_binding"</h3>
        <p>"父组件值：" {name}</p>
        // TODO: 将 name() 作为 value、set_name 封装为 on_change 传入
        <BindedInput value={name()} on_change=move |val: String| {
            set_name(val);
        }/>
        // 再次展示以验证双向绑定
        <p>"再次展示：" {name}</p>
    }
}

fn main() {
    mount_to_body(App);
}

// <details>
// 参考答案:
//
// use leptos::prelude::*;
//
// #[component]
// fn BindedInput<F>(value: String, on_change: F) -> impl IntoView
// where
//     F: Fn(String) + 'static,
// {
//     view! {
//         <input
//             type="text"
//             prop:value={value}
//             on:input=move |ev| {
//                 on_change(event_target_value(&ev));
//             }
//         />
//     }
// }
//
// #[component]
// fn App() -> impl IntoView {
//     let (name, set_name) = signal(String::from("Leptos"));
//
//     view! {
//         <h3>"练习 107: two_way_binding"</h3>
//         <p>"父组件值：" {name}</p>
//         <BindedInput value={name()} on_change=move |val: String| {
//             set_name(val);
//         }/>
//         <p>"再次展示：" {name}</p>
//     }
// }
//
// fn main() {
//     mount_to_body(App);
// }
// </details>
