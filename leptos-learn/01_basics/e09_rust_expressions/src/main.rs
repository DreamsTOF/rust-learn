// ============================================================
// 练习 e09: Rust 表达式嵌入 — 在 view! 中使用变量和计算
//
// 核心知识点:
//   - 变量插值: {变量名} 在 view! 中嵌入 Rust 变量
//   - 表达式嵌入: {表达式} 直接计算结果
//   - format! 宏: 格式化字符串
//
// 难度: ⭐⭐ (补全约 50%，关键位置有 TODO)
// ============================================================

use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    // TODO: 定义变量 name 和 year
    // 提示: let name = "Leptos"; let year = 2026;
    // 完成度: 变量定义已给出
    let name = todo!("给 name 赋值");
    let year = todo!("给 year 赋值");

    // TODO: 在 view! 中使用 {} 嵌入变量和表达式
    // 完成度: view! 内容已给出
    view! {
        <div>
            // 直接嵌入变量 — 使用 {name} 显示变量值
            <p>"Hello, " {/* TODO: 填入正确的变量名 */} "!"</p>

            // 嵌入变量 year
            <p>"今年是 " {/* TODO: 填入正确的变量名 */} " 年"</p>

            // 嵌入表达式 — {year + 1} 进行计算
            <p>"明年是 " {/* TODO: 填入表达式 */} " 年"</p>

            // 使用 format! 宏组合字符串
            <p>{/* TODO: 填入 format! 表达式 */}</p>
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
//     let name = "Leptos";
//     let year = 2026;
//     view! {
//         <div>
//             <p>"Hello, " {name} "!"</p>
//             <p>"今年是 " {year} " 年"</p>
//             <p>"明年是 " {year + 1} " 年"</p>
//             <p>{format!("{name} 版本 0.9 已发布")}</p>
//         </div>
//     }
// }
//
// fn main() {
//     mount_to_body(Exercise);
// }
//
// 知识点:
// 1. {变量名} 将 Rust 变量值插入到 view! 中
// 2. {表达式} 会先计算结果，再将结果渲染到 DOM
// 3. format! 宏可以在 {} 内部使用，生成格式化字符串
// 4. 任何实现了 IntoView 的类型都可以直接嵌入
// </details>
