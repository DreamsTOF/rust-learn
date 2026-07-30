use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    let cities = vec!["北京", "上海", "广州", "深圳", "杭州"];

    // TODO: 创建一个 selected_city signal (String 类型)
    // 提示: signal("北京")

    view! {
        <div>
            <h2>"练习 258 — 下拉选择"</h2>
            <select
                // TODO: 使用 prop:value 绑定到 selected_city signal
                // TODO: 绑定 on:change 事件，使用 event_target_value 更新 signal
            >
                {cities.into_iter().map(|city| {
                    view! { <option value={city}>{city}</option> }
                }).collect::<Vec<_>>()}
            </select>
            <p>
                "已选择: "
                // TODO: 显示已选中的城市
            </p>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
