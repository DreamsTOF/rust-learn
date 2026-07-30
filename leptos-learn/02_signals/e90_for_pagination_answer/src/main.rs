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
