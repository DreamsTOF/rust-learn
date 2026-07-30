// ============================================================
// 练习 e119: use_context — 子组件消费上下文值
//
// 核心知识点:
//   - use_context::<T>() 从组件树中查找最近祖先提供的 T 值
//   - 返回 Option<T>，未提供时返回 None
//   - 与 provide_context 配套使用
//
// 难度: ⭐⭐
// ============================================================

use leptos::prelude::*;

// TODO: 子组件用 use_context 获取用户名并显示
#[component]
fn UserGreeting() -> impl IntoView {
    let username = use_context::<String>()
        .expect("String context should be provided by parent");

    view! {
        <div style="border:1px solid #4caf50; padding:8px; margin:8px 0; border-radius:4px;">
            <p>"👋 欢迎, " {username} "!"</p>
        </div>
    }
}

#[component]
fn StatusBar() -> impl IntoView {
    // 同一个 context 可以在多个子组件中分别读取
    let username = use_context::<String>()
        .expect("String context should be provided by parent");

    view! {
        <div style="background:#f5f5f5; padding:4px 8px; border-radius:4px; font-size:0.85rem;">
            "已登录用户: " {username}
        </div>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    // TODO: 用 provide_context 提供 String 类型的上下文
    provide_context("Alice".to_string());

    view! {
        <div style="padding:8px; border:1px solid #ccc; border-radius:4px;">
            <h3>"use_context 示例"</h3>
            <p>"父组件通过 provide_context 注入了用户信息"</p>
            <UserGreeting/>
            <StatusBar/>
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
// fn UserGreeting() -> impl IntoView {
//     let username = use_context::<String>().expect("...");
//     view! { <p>"欢迎, " {username} "!"</p> }
// }
//
// #[component]
// fn StatusBar() -> impl IntoView {
//     let username = use_context::<String>().expect("...");
//     view! { <p>"用户: " {username}</p> }
// }
//
// #[component]
// fn Exercise() -> impl IntoView {
//     provide_context("Alice".to_string());
//     view! {
//         <div>
//             <h3>"use_context"</h3>
//             <UserGreeting/>
//             <StatusBar/>
//         </div>
//     }
// }
//
// fn main() { mount_to_body(Exercise); }
// ```
//
// ### 知识点
// - `use_context::<T>()` 在当前组件树中向上查找最近祖先的 `provide_context`
// - 返回 `Option<T>`：找到则 `Some(value)`，未找到则 `None`
// - 同一个 context 可在多个子组件中分别调用读取
// - 结合 `expect("...")` 可在开发阶段快速定位缺失 context 的问题
//
// </details>
