// ============================================================
// 练习 e122: 多类型 Context (multi_context)
//
// 核心知识点:
//   - 同时提供多个不同类型的 Context
//   - context 以类型区分，互不干扰
//
// 难度: ⭐⭐
// ============================================================

use leptos::prelude::*;

// TODO: 定义两个新类型包装，用于区分同类型的 Context
#[derive(Clone)]
struct UserName(String);

#[derive(Clone)]
struct UserAge(u32);

// TODO: 用 use_context 获取 UserName 和 UserAge 并显示
#[component]
fn UserProfile() -> impl IntoView {
    let name = use_context::<UserName>()
        .expect("UserName should be provided");
    let age = use_context::<UserAge>()
        .expect("UserAge should be provided");

    view! {
        <div style="border: 1px solid green; padding: 8px; margin: 8px 0;">
            <h3>"User Profile"</h3>
            <p>"Name: " {name.0.clone()}</p>
            <p>"Age: " {age.0}</p>
        </div>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    // TODO: 同时提供 UserName 和 UserAge
    provide_context(UserName("Alice".to_string()));
    provide_context(UserAge(30u32));

    view! {
        <div style="border: 1px solid gray; padding: 8px;">
            <h2>"Multi Context Demo"</h2>
            <p>"同时提供 UserName(String) 和 UserAge(u32) 两种 Context"</p>
            <UserProfile/>
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
// #[derive(Clone)]
// struct UserName(String);
// #[derive(Clone)]
// struct UserAge(u32);
//
// #[component]
// fn UserProfile() -> impl IntoView {
//     let name = use_context::<UserName>().expect("...");
//     let age = use_context::<UserAge>().expect("...");
//     view! {
//         <div>
//             <p>"Name: " {name.0.clone()}</p>
//             <p>"Age: " {age.0}</p>
//         </div>
//     }
// }
//
// #[component]
// fn Exercise() -> impl IntoView {
//     provide_context(UserName("Alice".to_string()));
//     provide_context(UserAge(30));
//     view! {
//         <div>
//             <UserProfile/>
//         </div>
//     }
// }
//
// fn main() { mount_to_body(Exercise); }
// ```
//
// ### 知识点
// - Context 以 Rust 类型为键，不同类型自动隔离
// - 提供多个 Context 只需多次调用 provide_context
// - 使用新类型 (newtype) 包装避免类型冲突
// - use_context 只返回与指定类型匹配的值
//
// </details>
