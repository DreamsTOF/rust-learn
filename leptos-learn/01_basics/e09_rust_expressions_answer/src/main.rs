// ============================================================
// 练习 e09: Rust 表达式嵌入 — 参考答案
//
// 核心知识点:
//   - 变量插值: {变量名} 在 view! 中嵌入 Rust 变量
//   - 表达式嵌入: {表达式} 直接计算结果
//   - format! 宏: 格式化字符串
// ============================================================

use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    let name = "Leptos";
    let year = 2026;

    view! {
        <div>
            <p>"Hello, " {name} "!"</p>
            <p>"今年是 " {year} " 年"</p>
            <p>"明年是 " {year + 1} " 年"</p>
            <p>{format!("{name} 版本 0.9 已发布")}</p>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
