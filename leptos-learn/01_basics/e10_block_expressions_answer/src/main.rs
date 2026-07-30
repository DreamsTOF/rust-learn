// ============================================================
// 练习 e10: 块级表达式 — 参考答案
//
// 核心知识点:
//   - 块表达式: { let x = ...; x + y }
//   - 块内多语句: 多条语句以分号分隔，最后一条作为返回值
//   - 块内条件: if/else 作为表达式返回值
// ============================================================

use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <div>
            <p>
                "计算 1 + 2 = "
                { let x = 1; let y = 2; x + y }
            </p>

            <p>
                "判断结果: "
                {
                    let score = 85;
                    if score >= 60 { "及格" } else { "不及格" }
                }
            </p>

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
