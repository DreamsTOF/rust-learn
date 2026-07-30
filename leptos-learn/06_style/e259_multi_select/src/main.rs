use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    // 目标: 创建一个多选列表
    //
    // 1. 定义编程语言列表: "HTML", "CSS", "JavaScript", "Rust", "Python"
    // 2. 使用 RwSignal::new(Vec::<String>::new()) 保存选中的选项
    // 3. 创建 <select multiple> 元素
    // 4. 在 on:change 事件中读取所有选中的选项
    //    提示:
    //    - event_target::<leptos::web_sys::HtmlSelectElement>(&ev)
    //    - .options() 返回 HtmlOptionsCollection
    //    - .get_with_index(i) 获取 HtmlOptionElement
    //    - opt.selected() 判断是否选中，opt.value() 获取值
    // 5. 将选中项显示在 <ul> 列表中

    view! {
        <div>
            <h2>"练习 259 — 多选列表"</h2>
            {}
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
