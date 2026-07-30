// ============================================================
// 练习 e120: context_type_safe — 类型安全的 Context
//
// 核心知识点:
//   - provide_context 以类型为键存储
//   - use_context 泛型参数必须严格匹配才能获取到值
//   - 类型不匹配时返回 None（类型安全）
//
// 难度: ⭐
// ============================================================

use leptos::prelude::*;

#[derive(Clone)]
struct UserId(u32);

#[component]
fn UserDisplay() -> impl IntoView {
    // TODO: 分别用不同类型读取 Context
    // UserId 类型匹配 — 应该能拿到
    let user_id = use_context::<UserId>();

    // &'static str 类型不匹配 — 应当拿到 None
    let wrong_type = use_context::<&'static str>();

    view! {
        <div style="border:1px solid #999; padding:8px; margin:8px 0; border-radius:4px;">
            <p><strong>"UserId context: "</strong>
                {match user_id {
                    Some(id) => format!("{}", id.0),
                    None => "(未找到)".to_string(),
                }}
            </p>
            <p><strong>"&'static str context: "</strong>
                {match wrong_type {
                    Some(s) => s,
                    None => "(未找到 — 类型不匹配)",
                }}
            </p>
        </div>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    // TODO: 提供 UserId 类型的 Context
    provide_context(UserId(42));

    view! {
        <div style="padding:8px;">
            <h3>"类型安全的 Context"</h3>
            <p>"provide_context(UserId(42)) — 尝试用不同类型读取"</p>
            <UserDisplay/>
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
// struct UserId(u32);
//
// #[component]
// fn UserDisplay() -> impl IntoView {
//     let user_id = use_context::<UserId>();
//     let wrong_type = use_context::<&'static str>();
//     view! {
//         <div>
//             <p>"UserId: " {match user_id {
//                 Some(id) => format!("{}", id.0),
//                 None => "(未找到)".to_string(),
//             }}</p>
//             <p>"&str: " {match wrong_type {
//                 Some(s) => s,
//                 None => "(未找到 — 类型不匹配)",
//             }}</p>
//         </div>
//     }
// }
//
// #[component]
// fn Exercise() -> impl IntoView {
//     provide_context(UserId(42));
//     view! {
//         <div>
//             <h3>"类型安全的 Context"</h3>
//             <UserDisplay/>
//         </div>
//     }
// }
//
// fn main() { mount_to_body(Exercise); }
// ```
//
// ### 知识点
// - Context 以 Rust 的具体类型为键，不同类型自动隔离
// - `use_context::<UserId>()` 只能读取到 `provide_context(UserId(...))` 的值
// - 类型不匹配时返回 `None`，不会类型转换或静默失败
// - 这是 Rust 类型系统的实际应用：类型安全在运行时也有保障
//
// </details>
