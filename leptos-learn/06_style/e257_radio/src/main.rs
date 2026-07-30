use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    // 选项列表
    let fruits = vec!["苹果", "香蕉", "橙子", "葡萄"];

    // TODO: 创建一个 selected signal (String 类型)
    // 提示: signal(String::new())

    view! {
        <div>
            <h2>"练习 257 — 单选按钮"</h2>
            <p>"请选择你最喜欢的水果："</p>
            {
                // TODO: 遍历 fruits，为每个选项创建 radio 按钮
                // - type="radio" name="fruit" value={水果名}
                // - 绑定 on:change 事件，将选中值写入 selected signal
            }
            <p>
                "已选择: "
                // TODO: 显示选中的值，如果未选择则显示 "(无)"
            </p>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
