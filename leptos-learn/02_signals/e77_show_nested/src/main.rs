// ============================================================
// 练习 e77: Show Nested — 组合条件 a && b
//
// 核心知识点:
//   - 在 when 中使用组合逻辑 `a && b`
//   - 两个信号同时为 true 才显示内容
//
// 难度: ⭐⭐ (TODO 约 50%)
// ============================================================

use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    // TODO: 创建两个布尔信号 admin 和 verified
    let (admin, set_admin) = signal(false);
    let (verified, set_verified) = signal(false);

    view! {
        <label>
            <input type="checkbox"
                prop:checked=move || admin.get()
                on:change=move |_| set_admin.update(|v| *v = !*v)
            />
            "管理员"
        </label>
        <label>
            <input type="checkbox"
                prop:checked=move || verified.get()
                on:change=move |_| set_verified.update(|v| *v = !*v)
            />
            "已验证"
        </label>
        // TODO: 组合条件 admin && verified
        <Show when=move || admin.get() && verified.get()>
            <p>"🔒 管理面板（仅管理员且已验证可见）"</p>
        </Show>
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
//     let (admin, set_admin) = signal(false);
//     let (verified, set_verified) = signal(false);
//
//     view! {
//         <label>
//             <input type="checkbox"
//                 prop:checked=move || admin.get()
//                 on:change=move |_| set_admin.update(|v| *v = !*v)
//             /> "管理员"
//         </label>
//         <label>
//             <input type="checkbox"
//                 prop:checked=move || verified.get()
//                 on:change=move |_| set_verified.update(|v| *v = !*v)
//             /> "已验证"
//         </label>
//         <Show when=move || admin.get() && verified.get()>
//             <p>"🔒 管理面板（仅管理员且已验证可见）"</p>
//         </Show>
//     }
// }
//
// fn main() {
//     mount_to_body(Exercise);
// }
//
// ### 知识点
// - Show 的 when 闭包内可以使用任意 Rust 逻辑组合条件
// - 两个复选框都勾选时才会显示管理面板
// - 响应式系统会自动追踪 admin 和 verified 两个信号
// </details>
