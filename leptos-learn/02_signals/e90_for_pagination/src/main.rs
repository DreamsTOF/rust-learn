// ============================================================
// 练习 e90: for_pagination — 列表分页
//
// 核心知识点:
//   - .skip().take() 从列表中截取子集
//   - 当前页状态 + 总页数计算
//
// 难度: ⭐⭐⭐ (TODO 约 50%)
// ============================================================

use leptos::prelude::*;

const ITEMS: &[&str] = &[
    "苹果", "香蕉", "樱桃", "龙眼", "榴莲",
    "葡萄", "猕猴桃", "柠檬", "芒果", "橙子",
    "木瓜", "桃子", "梨", "菠萝", "草莓",
];

#[component]
fn Exercise() -> impl IntoView {
    let page_size = 5;
    let total = ITEMS.len();
    let total_pages = (total + page_size - 1) / page_size;

    let (page, set_page) = signal(0);

    let page_items = move || {
        let start = page.get() * page_size;
        ITEMS.iter().skip(start).take(page_size).map(|s| s.to_string()).collect::<Vec<String>>()
    };

    view! {
        <h2>"水果列表（分页）"</h2>
        <p>
            {move || format!("第 {}/{} 页", page.get() + 1, total_pages)}
        </p>
        <ul>
            <For each=page_items key=|item| item.clone() let:item>
                <li>{item}</li>
            </For>
        </ul>
        <button
            on:click=move |_| set_page.update(|p| *p = p.saturating_sub(1))
            disabled=move || page.get() == 0
        >
            "上一页"
        </button>
        <button
            on:click=move |_| set_page.update(|p| *p = (*p + 1).min(total_pages - 1))
            disabled=move || page.get() >= total_pages - 1
        >
            "下一页"
        </button>
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
// const ITEMS: &[&str] = &[
//     "苹果", "香蕉", "樱桃", "龙眼", "榴莲",
//     "葡萄", "猕猴桃", "柠檬", "芒果", "橙子",
//     "木瓜", "桃子", "梨", "菠萝", "草莓",
// ];
//
// #[component]
// fn Exercise() -> impl IntoView {
//     let page_size = 5;
//     let total = ITEMS.len();
//     let total_pages = (total + page_size - 1) / page_size;
//     let (page, set_page) = signal(0);
//     let page_items = move || {
//         let start = page.get() * page_size;
//         ITEMS.iter().skip(start).take(page_size).map(|s| s.to_string()).collect::<Vec<String>>()
//     };
//
//     view! {
//         <h2>"水果列表（分页）"</h2>
//         <p>{move || format!("第 {}/{} 页", page.get() + 1, total_pages)}</p>
//         <ul>
//             <For each=page_items key=|item| item.clone() let:item>
//                 <li>{item}</li>
//             </For>
//         </ul>
//         <button on:click=move |_| set_page.update(|p| *p = p.saturating_sub(1)) disabled=move || page.get() == 0>"上一页"</button>
//         <button on:click=move |_| set_page.update(|p| *p = (*p + 1).min(total_pages - 1)) disabled=move || page.get() >= total_pages - 1>"下一页"</button>
//     }
// }
//
// fn main() { mount_to_body(Exercise); }
//
// ### 知识点
// - `.skip(n)` 跳过前 n 个元素
// - `.take(n)` 取接下来 n 个元素
// - 组合使用 `.skip().take()` 实现分页
// - `<For>` 的 `each` 接收闭包或信号，响应式更新列表
// - `saturating_sub` 防止无符号下溢
// </details>
