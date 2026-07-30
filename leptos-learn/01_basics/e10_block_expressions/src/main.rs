// ============================================================
// 练习 e10: 块级表达式 — 在 view! 中使用 Rust 块
//
// 核心知识点:
//   - 块表达式: { let x = ...; x + y }
//   - 块内多语句: 多条语句以分号分隔，最后一条作为返回值
//   - 块内条件: if/else 作为表达式返回值
//
// 难度: ⭐⭐ (补全约 50%，关键位置有 TODO)
// ============================================================

use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <div>
            // 块表达式: 在 {} 内定义变量并计算
            <p>
                "计算 1 + 2 = "
                { let x = 1; let y = 2; x + y }
            </p>

            // 块内使用 if/else 表达式 — 根据条件返回不同值
            <p>
                "判断结果: "
                {
                    let score = 85;
                    if score >= 60 { "及格" } else { "不及格" }
                }
            </p>

            // 块内多条语句 — 最后一条表达式作为返回值
            <p>
                "块表达式可以包含多条语句: "
                {
                    let a = 10;
                    let b = 20;
                    let c = a * b;
                    c.to_string()
                }
            </p>
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
// #[component]
// fn Exercise() -> impl IntoView {
//     view! {
//         <div>
//             <p>
//                 "计算 1 + 2 = "
//                 { let x = 1; let y = 2; x + y }
//             </p>
//             <p>
//                 "判断结果: "
//                 {
//                     let score = 85;
//                     if score >= 60 { "及格" } else { "不及格" }
//                 }
//             </p>
//             <p>
//                 "块表达式可以包含多条语句: "
//                 {
//                     let a = 10;
//                     let b = 20;
//                     let c = a * b;
//                     c.to_string()
//                 }
//             </p>
//         </div>
//     }
// }
//
// fn main() {
//     mount_to_body(Exercise);
// }
//
// 知识点:
// 1. 块表达式 { ... } 内的最后一条语句（不带分号）即为返回值
// 2. if/else 也是表达式，可以在块内作为返回值使用
// 3. 块内定义的变量作用域仅限于该块
// 4. 返回值需要实现 IntoView 或 Display trait
// </details>
