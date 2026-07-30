// ============================================================
// 练习 e130: NodeRef 基础 — 获取 DOM 元素引用
//
// 核心知识点:
//   - NodeRef<T>::new() 创建引用
//   - node_ref 属性绑定到元素
//   - .get() 获取 Option<&HtmlElement<T>>
//
// 难度: ⭐⭐ (补全关键位置)
// ============================================================

use leptos::prelude::*;
use leptos::html;

#[component]
fn Exercise() -> impl IntoView {
    // TODO: 创建一个 NodeRef<html::Div> 引用
    let div_ref: NodeRef<html::Div> = NodeRef::new();
    let (text, set_text) = signal(String::new());

    view! {
        <h2>"NodeRef 基础"</h2>
        // TODO: 使用 node_ref 属性将 div_ref 绑定到 <div>
        <div
            node_ref=div_ref
            style="padding: 1rem; border: 1px solid #ccc; border-radius: 4px; cursor: pointer;"
            on:click=move |_| {
                // TODO: 通过 div_ref.get() 访问 DOM 元素
                if let Some(el) = div_ref.get() {
                    let _ = set_text.set(el.text_content().unwrap_or_default());
                }
            }
        >
            "点击此方块"
        </div>
        <p>"上一次点击时的内容: " {text}</p>
    }
}

fn main() {
    mount_to_body(Exercise);
}

// <details>
// 参考答案 (去除注释后的纯净版本):
//
// use leptos::prelude::*;
// use leptos::html;
//
// #[component]
// fn Exercise() -> impl IntoView {
//     let div_ref: NodeRef<html::Div> = NodeRef::new();
//     let (text, set_text) = signal(String::new());
//     view! {
//         <h2>"NodeRef 基础"</h2>
//         <div
//             node_ref=div_ref
//             style="padding: 1rem; border: 1px solid #ccc; border-radius: 4px; cursor: pointer;"
//             on:click=move |_| {
//                 if let Some(el) = div_ref.get() {
//                     let _ = set_text.set(el.text_content().unwrap_or_default());
//                 }
//             }
//         >
//             "点击此方块"
//         </div>
//         <p>"上一次点击时的内容: " {text}</p>
//     }
// }
//
// fn main() {
//     mount_to_body(Exercise);
// }
//
// ### 知识点
// - NodeRef<T> 是 DOM 元素的零成本引用
// - 通过 node_ref 属性绑定到 JSX 元素
// - .get() 在元素挂载后返回 Some(&HtmlElement<T>)
// - 常用于: 测量尺寸、焦点管理、与第三方 JS 库集成
// </details>
