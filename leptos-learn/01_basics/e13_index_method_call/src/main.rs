use leptos::prelude::*;

// ============================================================
// 练习 e13 — 索引/方法调用
// 目标: 在 view! 中调用 Vec 的方法和索引
// 难度: ⭐⭐
// 核心知识点: { items.len() }, { items[0] }
// ============================================================

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <Exercise/> });
}
 
/// 使用 Vec 存储数据，在 view! 中调用 len() 和索引访问
#[component]
fn Exercise() -> impl IntoView {
    let items = vec!["Rust", "Leptos", "WASM"];

    view! {
        <div>
            <h2>"编程语言列表"</h2>
            <p>
                "共有 "
                // TODO: 将 "?" 替换为 items.len() 获取长度
                "?"
                " 门语言"
            </p>
            <p>
                "第一门语言: "
                // TODO: 将 "?" 替换为 items[0] 索引访问第一个元素
                "?"
            </p>
            <p>
                "第二门语言: "
                // TODO: 将 "?" 替换为 items[1] 索引访问第二个元素
                "?"
            </p>
        </div>
    }
}
