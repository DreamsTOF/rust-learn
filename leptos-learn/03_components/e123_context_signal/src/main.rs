// ============================================================
// 练习 e123: Context 传递信号 (context_signal)
//
// 核心知识点:
//   - 通过 Context 传递 RwSignal，实现共享可变状态
//   - 子组件可以读写父组件提供的信号
//
// 难度: ⭐⭐
// ============================================================

use leptos::prelude::*;

// TODO: 使用 use_context 获取共享的 RwSignal<i32> 实现计数
#[component]
fn CounterButton() -> impl IntoView {
    let count = use_context::<RwSignal<i32>>()
        .expect("RwSignal<i32> should be provided");

    view! {
        <div style="border: 1px solid blue; padding: 8px; margin: 8px 0;">
            <p>"Count: " {count}</p>
            // TODO: 点击按钮时增加计数
            <button on:click=move |_| count.update(|n| *n += 1)>"+1"</button>
            <button on:click=move |_| count.update(|n| *n -= 1)>"-1"</button>
        </div>
    }
}

#[component]
fn CounterDisplay() -> impl IntoView {
    let count = use_context::<RwSignal<i32>>()
        .expect("RwSignal<i32> should be provided");

    view! {
        <div style="border: 1px solid orange; padding: 8px; margin: 8px 0;">
            <p>"Current count (read-only view): " {count}</p>
        </div>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    // TODO: 创建 RwSignal 并通过 Context 共享
    let count = RwSignal::new(0);
    provide_context(count);

    view! {
        <div style="border: 1px solid gray; padding: 8px;">
            <h2>"Context Signal Demo"</h2>
            <p>"通过 Context 传递 RwSignal，多个子组件共享同一状态"</p>
            <CounterButton/>
            <CounterDisplay/>
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
// fn CounterButton() -> impl IntoView {
//     let count = use_context::<RwSignal<i32>>().expect("...");
//     view! {
//         <div>
//             <p>"Count: " {count}</p>
//             <button on:click=move |_| count.update(|n| *n += 1)>"+1"</button>
//             <button on:click=move |_| count.update(|n| *n -= 1)>"-1"</button>
//         </div>
//     }
// }
//
// #[component]
// fn CounterDisplay() -> impl IntoView {
//     let count = use_context::<RwSignal<i32>>().expect("...");
//     view! { <p>"Current count: " {count}</p> }
// }
//
// #[component]
// fn Exercise() -> impl IntoView {
//     let count = RwSignal::new(0);
//     provide_context(count);
//     view! {
//         <div>
//             <CounterButton/>
//             <CounterDisplay/>
//         </div>
//     }
// }
//
// fn main() { mount_to_body(Exercise); }
// ```
//
// ### 知识点
// - RwSignal 同时实现读和写，可通过 Context 共享
// - 多个子组件可以访问同一个 RwSignal，自动同步
// - count.update(|n| *n += 1) 是安全的可变更新方式
// - 在 view! 中直接使用 {count} 读取当前值 (Fn 调用)
//
// </details>
