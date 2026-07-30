// ============================================================
// 练习 e106: 带参数的回调 (Callback with Parameters)
//
// 核心知识点:
//   - 使用泛型闭包 F: Fn(String) 作为组件 prop
//   - on:input 事件处理与 event_target_value
//
// 难度: ⭐⭐
// ============================================================

use leptos::prelude::*;

// 子组件：接收 on_input 回调，在输入时传递当前值
#[component]
fn SearchInput<F>(on_input: F) -> impl IntoView
where
    F: Fn(String) + 'static,
{
    view! {
        <input
            type="text"
            placeholder="在此输入..."
            // TODO: 在 on:input 中提取 ev 的目标值并调用 on_input
            on:input=move |ev| {
                on_input(event_target_value(&ev));
            }
        />
    }
}

#[component]
fn App() -> impl IntoView {
    // 保存用户输入的内容
    let (text, set_text) = signal(String::new());

    view! {
        <h3>"练习 106: callback_params"</h3>
        // TODO: 为 SearchInput 传入 on_input 回调
        <SearchInput on_input=move |val: String| {
            set_text(val);
        }/>
        <p>"你输入了: " {text}</p>
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
// fn SearchInput<F>(on_input: F) -> impl IntoView
// where
//     F: Fn(String) + 'static,
// {
//     view! {
//         <input
//             type="text"
//             placeholder="在此输入..."
//             on:input=move |ev| {
//                 on_input(event_target_value(&ev));
//             }
//         />
//     }
// }
//
// #[component]
// fn App() -> impl IntoView {
//     let (text, set_text) = signal(String::new());
//
//     view! {
//         <h3>"练习 106: callback_params"</h3>
//         <SearchInput on_input=move |val: String| {
//             set_text(val);
//         }/>
//         <p>"你输入了: " {text}</p>
//     }
// }
//
// fn main() {
//     mount_to_body(App);
// }
// </details>
