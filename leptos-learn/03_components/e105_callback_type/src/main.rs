// ============================================================
// 练习 e105: Callback Type — 使用 Callback 类型
//
// 核心知识点:
//   - Callback::new() 创建类型安全的回调
//   - Callback<T> 是 Copy + Clone 的回调包装
//   - 通过 .run(input) 调用回调
//
// 难度: ⭐⭐
// ============================================================

use leptos::prelude::*;

// ActionButton 接收 Callback<()> 类型
// Callback<In, Out = ()> 是 Send + Sync 的线程安全回调包装
// 相比泛型闭包，Callback 是具体类型，适合作为公开 API 的 prop
#[component]
fn ActionButton(action: Callback<()>) -> impl IntoView {
    view! {
        <button
            style="background:#3498db;color:white;border:none;padding:8px 16px;border-radius:4px;cursor:pointer;"
            on:click=move |_| action.run(())
        >
            "执行操作"
        </button>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    let (count, set_count) = signal(0);

    // TODO: 使用 Callback::new() 创建回调
    // Callback::new() 要求闭包为 Fn(In) -> Out + Send + Sync + 'static
    let increment = Callback::new(move |_| {
        set_count.update(|n| *n += 1);
    });

    view! {
        <p>"次数: " {count}</p>
        <ActionButton action=increment />
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
// fn ActionButton(action: Callback<()>) -> impl IntoView {
//     view! {
//         <button
//             style="background:#3498db;color:white;border:none;padding:8px 16px;border-radius:4px;cursor:pointer;"
//             on:click=move |_| action.run(())
//         >
//             "执行操作"
//         </button>
//     }
// }
//
// #[component]
// fn Exercise() -> impl IntoView {
//     let (count, set_count) = signal(0);
//     let increment = Callback::new(move |_| {
//         set_count.update(|n| *n += 1);
//     });
//     view! {
//         <p>"次数: " {count}</p>
//         <ActionButton action=increment />
//     }
// }
//
// fn main() {
//     mount_to_body(Exercise);
// }
// </details>
