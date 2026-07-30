// ============================================================
// 练习 e346: Signal vs Global — 比较 RwSignal 本地状态 vs create_context 全局状态
//
// 核心知识点:
//   - RwSignal::new() 创建本地状态，每个组件实例独立
//   - provide_context / expect_context 实现全局状态共享
//   - 对比两种状态管理方式的适用场景
//
// 难度: ⭐⭐ (关键位置有 TODO，补全约 50%)
// ============================================================

use leptos::prelude::*;

// TODO: 创建一个全局计数器组件 GlobalCounter
// 它通过 expect_context 获取共享的 RwSignal<i32>，并显示/递增计数值
// 提示: expect_context::<RwSignal<i32>>() 从上下文中获取共享信号

// TODO: 创建一个本地计数器组件 LocalCounter
// 它在组件内部使用 RwSignal::new(0) 创建独立的本地状态

#[component]
fn Exercise() -> impl IntoView {
    // TODO: 使用 provide_context 注入共享的 RwSignal<i32>
    // 提示: provide_context(RwSignal::new(0));

    view! {
        <div>
            <h2>"全局状态（共享）"</h2>
            // TODO: 添加两个 GlobalCounter 实例并观察共享效果

            <h2>"本地状态（独立）"</h2>
            // TODO: 添加两个 LocalCounter 实例并观察独立效果
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
// fn GlobalCounter() -> impl IntoView {
//     let count = expect_context::<RwSignal<i32>>();
//     view! {
//         <div>
//             <p>"全局计数: " {move || count.get()}</p>
//             <button on:click=move |_| count.update(|n| *n += 1)>"+1"</button>
//         </div>
//     }
// }
//
// #[component]
// fn LocalCounter() -> impl IntoView {
//     let count = RwSignal::new(0);
//     view! {
//         <div>
//             <p>"本地计数: " {move || count.get()}</p>
//             <button on:click=move |_| count.update(|n| *n += 1)>"+1"</button>
//         </div>
//     }
// }
//
// #[component]
// fn Exercise() -> impl IntoView {
//     provide_context(RwSignal::new(0));
//     view! {
//         <div>
//             <h2>"全局状态（共享）"</h2>
//             <GlobalCounter/>
//             <GlobalCounter/>
//             <h2>"本地状态（独立）"</h2>
//             <LocalCounter/>
//             <LocalCounter/>
//         </div>
//     }
// }
//
// fn main() {
//     mount_to_body(Exercise);
// }
//
// ### 知识点
// - `RwSignal::new(value)` 在 arena 中分配一个可读可写的信号
// - `provide_context(value)` 将值注入到当前组件及其子组件的上下文中
// - `expect_context::<T>()` 从上下文中获取指定类型的值（如果没有则 panic）
// - 全局状态：所有子组件共享同一个 RwSignal 实例，一处修改处处更新
// - 本地状态：每个组件实例有独立的 RwSignal，互不影响
// - 选择原则：组件间确实需要共享状态时用 context，否则优先用本地信号
// </details>
