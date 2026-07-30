// ============================================================
// 练习 e127: Context Memo — 通过 Context 提供派生状态
//
// 核心知识点:
//   - provide_context 传递 Memo
//   - use_context 消费派生状态
//   - Context + Memo 组合模式
//
// 难度: ⭐⭐⭐ (补全关键位置)
// ============================================================

use leptos::prelude::*;

#[component]
fn Child() -> impl IntoView {
    // TODO: 从 context 中获取 Memo<i32> 派生状态
    let doubled = use_context::<Memo<i32>>().expect("doubled Memo 应通过 context 提供");

    view! {
        <p>"派生值 (Memo): " {doubled}</p>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    // TODO: 创建原始信号和派生 Memo，将 Memo 通过 context 提供
    let (count, set_count) = signal(1);
    let doubled = Memo::new(move |_| count.get() * 2);

    provide_context(doubled);

    view! {
        <h2>"Context + Memo"</h2>
        <p>"原始值: " {count}</p>
        <button on:click=move |_| set_count.update(|c| *c += 1)>"+1"</button>
        <hr/>
        <Child/>
    }
}

fn main() {
    mount_to_body(Exercise);
}

// <details>
// 参考答案 (去除注释后的纯净版本):
//
// use leptos::prelude::*;
//
// #[component]
// fn Child() -> impl IntoView {
//     let doubled = use_context::<Memo<i32>>().expect("doubled Memo 应通过 context 提供");
//     view! { <p>"派生值 (Memo): " {doubled}</p> }
// }
//
// #[component]
// fn Exercise() -> impl IntoView {
//     let (count, set_count) = signal(1);
//     let doubled = Memo::new(move |_| count.get() * 2);
//     provide_context(doubled);
//     view! {
//         <h2>"Context + Memo"</h2>
//         <p>"原始值: " {count}</p>
//         <button on:click=move |_| set_count.update(|c| *c += 1)>"+1"</button>
//         <hr/>
//         <Child/>
//     }
// }
//
// fn main() {
//     mount_to_body(Exercise);
// }
//
// ### 知识点
// - Memo 是惰性求值的派生状态，自动跟踪依赖并缓存
// - 通过 Context 传递 Memo 让任意后代组件共享派生状态
// - 这是"提供派生状态"的推荐模式
// </details>
