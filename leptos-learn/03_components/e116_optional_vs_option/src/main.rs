// ============================================================
// 练习 e116: optional_vs_option — #[prop(optional)] vs Option<T>
//
// 核心知识点:
//   - #[prop(optional)] 让 Option 字段在调用处可选（可省略）
//   - 无 #[prop(optional)] 的 Option 字段调用时必须传参
//   - 前者写起来更简洁，后者在需要显式传 None 时有用
//
// 难度: ⭐⭐
// ============================================================

use leptos::prelude::*;

// TODO: 定义 DualProps 组件，对比两种可选 prop 写法
#[component]
fn DualProps(
    /// #[prop(optional)]: 调用时可省略该 prop，内部收到 None
    #[prop(optional)]
    optional_msg: Option<&'static str>,
    /// 无 #[prop(optional)]: 调用时必须传 Some(...) 或 None
    explicit_msg: Option<&'static str>,
) -> impl IntoView {
    view! {
        <div style="border:1px solid #999; padding:8px; margin:8px 0; border-radius:4px;">
            <p><strong>"optional_msg:"</strong> {optional_msg.unwrap_or("(未提供 — None)")}</p>
            <p><strong>"explicit_msg:"</strong> {explicit_msg.unwrap_or("(未提供 — None)")}</p>
        </div>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <div style="padding:8px;">
            <h3>"#[prop(optional)] vs Option<T>"</h3>

            // 情况 1: 省略 optional_msg，只传 explicit_msg
            <DualProps explicit_msg=Some("必须用 Some 包裹") />

            // 情况 2: 两个都传
            <DualProps
                optional_msg="省略式传入"
                explicit_msg=Some("仍需 Some 包裹")
            />

            // 情况 3: explicit_msg 显式传 None
            <DualProps explicit_msg=None />

            <p style="margin-top:16px; font-size:0.9em; color:#666;">
                "注意: optional_msg 可整行省略，explicit_msg 则不能省略"
            </p>
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
// fn DualProps(
//     #[prop(optional)]
//     optional_msg: Option<&'static str>,
//     explicit_msg: Option<&'static str>,
// ) -> impl IntoView {
//     view! {
//         <div>
//             <p>"optional_msg: " {optional_msg.unwrap_or("(None)")}</p>
//             <p>"explicit_msg: " {explicit_msg.unwrap_or("(None)")}</p>
//         </div>
//     }
// }
//
// #[component]
// fn Exercise() -> impl IntoView {
//     view! {
//         <div>
//             <h3>"#[prop(optional)] vs Option<T>"</h3>
//             <DualProps explicit_msg=Some("必须用 Some 包裹") />
//             <DualProps
//                 optional_msg="省略式传入"
//                 explicit_msg=Some("仍需 Some 包裹")
//             />
//             <DualProps explicit_msg=None />
//         </div>
//     }
// }
//
// fn main() { mount_to_body(Exercise); }
// ```
//
// ### 知识点
// - `#[prop(optional)]` + `Option<T>`: 调用方可完全省略该 prop
// - 无 `#[prop(optional)]` 的 `Option<T>`: 调用方必须显式传值
// - 两者最终组件内部拿到的都是 `Option<T>`
// - 选用原则: 绝大多数情况用 `#[prop(optional)]`；需要调用方明确选择时才用裸 `Option<T>`
//
// </details>
