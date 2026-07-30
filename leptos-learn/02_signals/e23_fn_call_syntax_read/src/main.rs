// ============================================================
// 练习 23: fn_call_syntax_read
//
// 目标: 演示 count() 等价于 count.get()
//
// 难度: ⭐
// 核心知识点: 函数调用语法读取
// ============================================================
use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    let (count, _set_count) = signal(100);

    view! {
        <div>
            <p>"count() = " {count()}</p>
            <p>"count.get() = " {count.get()}</p>
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
// view! {
//     <p>{count()}</p>
//     <p>{count.get()}</p>
// }
// ```
//
// ### 知识点
// - `count()` 是 `count.get()` 的语法糖，二者完全等价
// - 函数调用语法更简洁，是 leptos 中的惯用写法
//
// </details>
