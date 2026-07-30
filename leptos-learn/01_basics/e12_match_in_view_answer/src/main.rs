// ============================================================
// 练习 e12: match 在 view 中 — 参考答案
//
// 核心知识点:
//   - { match x { 1 => "一", _ => "其他" } } 在 view! 中嵌入模式匹配
//   - match 的所有分支必须返回相同类型（此处均为 &str）
//   - 通配符 _ 处理所有未列出的值，保证匹配穷尽性
// ============================================================

use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    let (num, set_num) = signal(1);

    view! {
        <div>
            <p>
                "数字 "
                {num}
                " 对应中文: "
                {
                    match num() {
                        1 => "一",
                        2 => "二",
                        3 => "三",
                        _ => "其他",
                    }
                }
            </p>
            <button on:click=move |_| set_num(1)>"设置 1"</button>
            <button on:click=move |_| set_num(2)>"设置 2"</button>
            <button on:click=move |_| set_num(3)>"设置 3"</button>
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(Exercise);
}
