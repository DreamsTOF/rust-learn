// ============================================================
// 练习 e126: 全局状态 — AppState 全局单例
//
// 核心知识点:
//   - provide_context / use_context
//   - 全局状态管理模式: 根组件提供，任意后代组件消费
//
// 难度: ⭐⭐⭐ (补全关键位置)
// ============================================================

use leptos::prelude::*;

#[derive(Clone)]
struct AppState {
    count: RwSignal<i32>,
}

impl AppState {
    fn new() -> Self {
        Self {
            count: RwSignal::new(0),
        }
    }
}

#[component]
fn Counter() -> impl IntoView {
    // TODO: 从 context 中获取 AppState
    let state = use_context::<AppState>().expect("AppState 应在根组件提供");

    view! {
        <p>"全局计数: " {state.count}</p>
        <button on:click=move |_| state.count.update(|c| *c += 1)>"+1"</button>
        <button on:click=move |_| state.count.update(|c| *c -= 1)>"-1"</button>
    }
}

#[component]
fn Display() -> impl IntoView {
    // TODO: 从 context 中获取 AppState，显示派生值
    let state = use_context::<AppState>().expect("AppState 应在根组件提供");

    view! {
        <p>"当前值 × 2 = " {move || state.count.get() * 2}</p>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    // TODO: 创建 AppState 并通过 context 提供给子组件
    let state = AppState::new();
    provide_context(state);

    view! {
        <h2>"全局状态管理模式"</h2>
        <Counter/>
        <Display/>
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
// #[derive(Clone)]
// struct AppState {
//     count: RwSignal<i32>,
// }
//
// impl AppState {
//     fn new() -> Self {
//         Self { count: RwSignal::new(0) }
//     }
// }
//
// #[component]
// fn Counter() -> impl IntoView {
//     let state = use_context::<AppState>().expect("AppState 应在根组件提供");
//     view! {
//         <p>"全局计数: " {state.count}</p>
//         <button on:click=move |_| state.count.update(|c| *c += 1)>"+1"</button>
//         <button on:click=move |_| state.count.update(|c| *c -= 1)>"-1"</button>
//     }
// }
//
// #[component]
// fn Display() -> impl IntoView {
//     let state = use_context::<AppState>().expect("AppState 应在根组件提供");
//     view! { <p>"当前值 × 2 = " {move || state.count.get() * 2}</p> }
// }
//
// #[component]
// fn Exercise() -> impl IntoView {
//     let state = AppState::new();
//     provide_context(state);
//     view! {
//         <h2>"全局状态管理模式"</h2>
//         <Counter/>
//         <Display/>
//     }
// }
//
// fn main() {
//     mount_to_body(Exercise);
// }
//
// ### 知识点
// - provide_context 在组件树中按类型注册一个值
// - use_context 在任意后代组件中检索该值 (返回 Option<T>)
// - 适用于全局状态 (用户信息、主题、语言)
// - 同类型只能注册一个值 (底层: HashMap<TypeId, Box<dyn Any>>)
// </details>
