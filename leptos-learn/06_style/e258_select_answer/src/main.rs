use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    let cities = vec!["北京", "上海", "广州", "深圳", "杭州"];
    let (selected_city, set_selected_city) = signal("北京".to_string());

    view! {
        <div>
            <h2>"练习 258 — 下拉选择"</h2>
            <select
                prop:value={selected_city}
                on:change=move|ev| set_selected_city.set(event_target_value(&ev))
            >
                {cities.into_iter().map(|city| {
                    view! { <option value={city}>{city}</option> }
                }).collect::<Vec<_>>()}
            </select>
            <p>"已选择: " {move || selected_city.get()}</p>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
