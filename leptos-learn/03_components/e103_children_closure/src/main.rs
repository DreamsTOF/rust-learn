// ============================================================
// 练习 e103: Children Closure — 子节点作为闭包动态渲染
//
// 核心知识点:
//   - ChildrenFn 可多次调用（区别于 Children 的 FnOnce）
//   - 适合需要将同一组 children 渲染到多处的场景
//
// 难度: ⭐⭐⭐
// ============================================================

use leptos::prelude::*;

// FlexBox 将 children 渲染到两个独立的区域中
// 使用 ChildrenFn 而非 Children，因为需要调用两次 children()
//
// 如果换成 children: Children，编译器会报错：
// "`Box<dyn FnOnce() -> AnyView + Send>` cannot be called more than once"
#[component]
fn FlexBox(children: ChildrenFn) -> impl IntoView {
    view! {
        <div style="display:flex;gap:10px;padding:10px;border:2px solid #e67e22;border-radius:8px;">
            <div style="flex:1;background:#fdf2e9;padding:8px;border-radius:4px;">
                <p><strong>"区域 A"</strong></p>
                {children()}
            </div>
            <div style="flex:1;background:#fef9e7;padding:8px;border-radius:4px;">
                <p><strong>"区域 B"</strong></p>
                {children()}
            </div>
        </div>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    let (count, set_count) = signal(0);
    view! {
        <button on:click=move |_| set_count.update(|n| *n += 1)>
            "点击增加: " {count}
        </button>
        <FlexBox>
            <p>"计数 = " {move || count.get()}</p>
            <p>"同一 children 渲染到两个区域"</p>
        </FlexBox>
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
// fn FlexBox(children: ChildrenFn) -> impl IntoView {
//     view! {
//         <div style="display:flex;gap:10px;padding:10px;border:2px solid #e67e22;border-radius:8px;">
//             <div style="flex:1;background:#fdf2e9;padding:8px;border-radius:4px;">
//                 <p><strong>"区域 A"</strong></p>
//                 {children()}
//             </div>
//             <div style="flex:1;background:#fef9e7;padding:8px;border-radius:4px;">
//                 <p><strong>"区域 B"</strong></p>
//                 {children()}
//             </div>
//         </div>
//     }
// }
//
// #[component]
// fn Exercise() -> impl IntoView {
//     let (count, set_count) = signal(0);
//     view! {
//         <button on:click=move |_| set_count.update(|n| *n += 1)>
//             "点击增加: " {count}
//         </button>
//         <FlexBox>
//             <p>"计数 = " {move || count.get()}</p>
//             <p>"同一 children 渲染到两个区域"</p>
//         </FlexBox>
//     }
// }
//
// fn main() {
//     mount_to_body(Exercise);
// }
// </details>
