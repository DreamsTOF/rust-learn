// ============================================================
// 练习 e18: view! 多根节点 Fragment
//
// 核心知识点:
//   - <></> Fragment 语法
//   - Fragment 嵌套
//   - 组件中多个根节点返回
//
// 难度: ⭐ (填空题 — 每行都有 TODO 指引)
// ============================================================

use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    view! {
        // TODO: 方式一 — 使用 <></> 返回多个根节点，Fragment 不会产生额外 DOM 节点
        <>
            // TODO: 添加 h2 和两个 p 元素
        </>

        // TODO: 方式二 — Fragment 可以嵌套使用
        <>
            // TODO: 添加 h2 和嵌套 Fragment
        </>

        // TODO: 方式三 — Fragment 中可以放 0 个、1 个或多个元素
        <>
            // TODO: 添加 h2 和一个空 Fragment <></>
        </>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(Exercise);
}
