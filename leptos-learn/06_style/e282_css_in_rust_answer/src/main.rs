use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    let (hue, set_hue) = signal(0);
    let (saturation, set_saturation) = signal(50);
    let (lightness, set_lightness) = signal(50);

    let style = move || {
        format!(
            "background-color: hsl({}, {}%, {}%); \
             width: 200px; height: 200px; \
             border-radius: 8px; \
             transition: background-color 0.3s ease;",
            hue.get(), saturation.get(), lightness.get()
        )
    };

    view! {
        <div>
            <h2>"CSS in Rust — 动态样式"</h2>
            <div>
                <label>"色相 (Hue): " {hue}
                    <input type="range" min="0" max="360" value={hue}
                        on:input=move |ev| set_hue.set(event_target_value(&ev).parse::<i32>().unwrap_or(0)) />
                </label>
            </div>
            <div>
                <label>"饱和度 (Saturation): " {saturation}
                    <input type="range" min="0" max="100" value={saturation}
                        on:input=move |ev| set_saturation.set(event_target_value(&ev).parse::<i32>().unwrap_or(50)) />
                </label>
            </div>
            <div>
                <label>"明度 (Lightness): " {lightness}
                    <input type="range" min="0" max="100" value={lightness}
                        on:input=move |ev| set_lightness.set(event_target_value(&ev).parse::<i32>().unwrap_or(50)) />
                </label>
            </div>
            <div style={style}></div>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
