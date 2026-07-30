// ============================================================
// 练习 e07: Fragment 语法 — 多根节点渲染
//
// 核心知识点:
//   - Fragment: <></> 包裹多个兄弟根节点
//   - view! 宏支持多根节点返回
//
// 难度: ⭐⭐ (补全约 50%，关键位置有 TODO)
// ============================================================

use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    // TODO: 使用 Fragment <></> 返回多个并列的根节点
    // 提示: 将多个元素放在 <> 和 </> 之间即可
    // 完成度: view! 宏内容已给出
    view! {
        // 方式一：使用 <></> 包裹多个根节点
       <>
           /* TODO: 在 Fragment 内添加 <h2> 标题和多个 <p> 段落 */
       </>
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
//     view! {
//         <>
//             <h2>"Fragment 语法"</h2>
//             <p>"这是第一个段落"</p>
//             <p>"这是第二个段落"</p>
//         </>
//     }
// }
//
// fn main() {
//     mount_to_body(Exercise);
// }
//
// 知识点:
// 1. Fragment (<></>) 是一个轻量容器，不会在 DOM 中产生额外节点
// 2. 当你需要返回多个根元素但不想用 <div> 包裹时，使用 Fragment
// 3. Leptos view! 宏直接写多个并列元素也会自动使用 Fragment
// </details>
