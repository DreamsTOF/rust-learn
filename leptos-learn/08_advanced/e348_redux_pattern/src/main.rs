// ============================================================
// 练习 e348: Redux 模式 — Action + Reducer + Store
//
// 核心知识点:
//   - 定义 Action 枚举描述所有可能的状态变更
//   - 实现 Reducer 函数 (state, action) -> new_state
//   - 用 RwSignal 作为 Store，dispatch 函数封装更新逻辑
//
// 难度: ⭐⭐ (关键位置有 TODO，补全约 50%)
// ============================================================

use leptos::prelude::*;

#[derive(Clone, Copy)]
enum CounterAction {
    Increment,
    Decrement,
    Reset(i32),
}

fn counter_reducer(state: i32, action: CounterAction) -> i32 {
    match action {
        CounterAction::Increment => state + 1,
        CounterAction::Decrement => state - 1,
        CounterAction::Reset(value) => value,
    }
}

#[component]
fn Exercise() -> impl IntoView {
    let store = RwSignal::new(0);

    let dispatch = move |action: CounterAction| {
        store.update(|state| {
            *state = counter_reducer(*state, action);
        });
    };

    view! {
        <div>
            <h2>"Redux 风格计数器"</h2>
            <p>"计数: " {move || store.get()}</p>
            <button on:click=move |_| dispatch(CounterAction::Increment)>"+1"</button>
            <button on:click=move |_| dispatch(CounterAction::Decrement)>"-1"</button>
            <button on:click=move |_| dispatch(CounterAction::Reset(0))>"重置为 0"</button>
            <button on:click=move |_| dispatch(CounterAction::Reset(100))>"重置为 100"</button>
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
// #[derive(Clone, Copy)]
// enum CounterAction {
//     Increment,
//     Decrement,
//     Reset(i32),
// }
//
// fn counter_reducer(state: i32, action: CounterAction) -> i32 {
//     match action {
//         CounterAction::Increment => state + 1,
//         CounterAction::Decrement => state - 1,
//         CounterAction::Reset(value) => value,
//     }
// }
//
// #[component]
// fn Exercise() -> impl IntoView {
//     let store = RwSignal::new(0);
//
//     let dispatch = move |action: CounterAction| {
//         store.update(|state| {
//             *state = counter_reducer(*state, action);
//         });
//     };
//
//     view! {
//         <div>
//             <h2>"Redux 风格计数器"</h2>
//             <p>"计数: " {move || store.get()}</p>
//             <button on:click=move |_| dispatch(CounterAction::Increment)>"+1"</button>
//             <button on:click=move |_| dispatch(CounterAction::Decrement)>"-1"</button>
//             <button on:click=move |_| dispatch(CounterAction::Reset(0))>"重置为 0"</button>
//             <button on:click=move |_| dispatch(CounterAction::Reset(100))>"重置为 100"</button>
//         </div>
//     }
// }
//
// fn main() {
//     mount_to_body(Exercise);
// }
//
// ### 知识点
// - Action 枚举：集中描述所有允许的状态变更
// - Reducer 函数：纯函数 (state, action) -> state，不含副作用
// - Store：用 RwSignal 持有当前状态
// - Dispatch：封装 store.update + reducer 调用
// - 优势：状态变更集中管理、可追溯、易测试
// - 注意：在 Leptos 中不需全局 Store，RwSignal + 闭包即可
// </details>
