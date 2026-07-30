use leptos::prelude::*;

async fn fetch_temperature(city: &str) -> f64 {
    match city {
        "北京" => 26.5,
        "上海" => 30.2,
        "广州" => 33.8,
        _ => 20.0,
    }
}

#[component]
fn Exercise() -> impl IntoView {
    let (city, set_city) = signal("北京".to_string());

    let temp_resource = LocalResource::new(move || {
        let city = city.get();
        async move { fetch_temperature(&city).await }
    });

    view! {
        <div>
            <h2>"Resource.map() 示例"</h2>
            <select on:change:target=move |ev| set_city.set(ev.target().value())>
                <option value="北京">"北京"</option>
                <option value="上海">"上海"</option>
                <option value="广州">"广州"</option>
            </select>

            <p>
                "温度: "
                {move || temp_resource
                    .map(|t| format!("{:.1}°C", t))
                    .unwrap_or_else(|| "加载中...".to_string())
                }
            </p>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
