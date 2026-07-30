// ============================================================
// 练习 e33: multi_signal — 多信号创建与操作
//
// 核心知识点:
//   - 同时管理多个不同类型的信号
//   - 每种信号独立追踪、独立更新
//
// 难度: ⭐
// ============================================================

use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    // TODO: 创建两个信号 — count (i32) 和 name (String)
    let (count, set_count) = signal(0);
    let (name, set_name) = signal("Leptos".to_string());

    let increment = move |_| {
        // TODO: 将 count 加 1
        set_count.update(|n| *n += 1);
    };

    let toggle_name = move |_| {
        // TODO: 在 "Leptos" 和 "Rust" 之间切换 name
        set_name.update(|n| {
            *n = if n == "Leptos" {
                "Rust".to_string()
            } else {
                "Leptos".to_string()
            };
        });
    };

    view! {
        <div>
            <p>"count: " {count}</p>
            <p>"name: " {name}</p>
            <button on:click=increment>"count +1"</button>
            <button on:click=toggle_name>"切换 name"</button>
        </div>
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
// fn Exercise() -> impl IntoView {
//     let (count, set_count) = signal(0);
//     let (name, set_name) = signal("Leptos".to_string());
//
//     let increment = move |_| set_count.update(|n| *n += 1);
//     let toggle_name = move |_| {
//         set_name.update(|n| {
//             *n = if n == "Leptos" {
//                 "Rust".to_string()
//             } else {
//                 "Leptos".to_string()
//             };
//         });
//     };
//
//     view! {
//         <div>
//             <p>"count: " {count}</p>
//             <p>"name: " {name}</p>
//             <button on:click=increment>"count +1"</button>
//             <button on:click=toggle_name>"切换 name"</button>
//         </div>
//     }
// }
//
// fn main() {
//     mount_to_body(Exercise);
// }
//
// 知识点:
// 1. 可同时创建任意多个 signal，类型可以各不相同
// 2. 每个信号独立追踪依赖，互不干扰
// 3. 信号的类型由初始值自动推导
// </details>
