// ============================================================
// 练习 e34: type_inference — Rust 类型推断与信号泛型
//
// 核心知识点:
//   - 从初始值隐式推导类型
//   - 泛型参数显式指定 ::<T>（turbofish）
//   - 类型注解声明
//   - &str 与 String 的推断差异
//
// 难度: ⭐⭐
// ============================================================

use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    // 方式 1：隐式推导 — 从初始值 0 推断为 i32
    let (count, set_count) = signal(0);

    // 方式 2：泛型参数 — turbofish 显式指定 ::<i32>
    let (_count2, _set2) = signal::<i32>(0);

    // 方式 3：类型注解 — 为变量标注 (ReadSignal<i32>, WriteSignal<i32>)
    let (count_anno, set_count_anno): (ReadSignal<i32>, WriteSignal<i32>) = signal(42);

    // 方式 4：字符串字面量 — 推断为 &str（不是 String）
    let (text, set_text) = signal("hello");

    let do_update = move |_| {
        set_count.update(|n| *n += 1);
        set_count_anno.update(|n| *n += 1);
        set_text.set("world");
    };

    view! {
        <div>
            <p>"隐式 count: " {count}</p>
            <p>"注解 count_anno: " {count_anno}</p>
            <p>"文字 text: " {text}</p>
            <button on:click=do_update>"全部更新"</button>
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
//     let (_count2, _set2) = signal::<i32>(0);
//     let (count_anno, set_count_anno): (ReadSignal<i32>, WriteSignal<i32>) = signal(42);
//     let (text, set_text) = signal("hello");
//
//     let do_update = move |_| {
//         set_count.update(|n| *n += 1);
//         set_count_anno.update(|n| *n += 1);
//         set_text.set("world");
//     };
//
//     view! {
//         <div>
//             <p>"隐式 count: " {count}</p>
//             <p>"注解 count_anno: " {count_anno}</p>
//             <p>"文字 text: " {text}</p>
//             <button on:click=do_update>"全部更新"</button>
//         </div>
//     }
// }
//
// fn main() {
//     mount_to_body(Exercise);
// }
//
// 知识点:
// 1. signal(0) 通过初始值 0 自动推导出 i32
// 2. signal::<i32>(0) 使用 turbofish 显式指定泛型参数
// 3. (ReadSignal<i32>, WriteSignal<i32>) 类型注解显式声明
// 4. signal("hello") 推断为 &str（静态字符串字面量类型）
// </details>
