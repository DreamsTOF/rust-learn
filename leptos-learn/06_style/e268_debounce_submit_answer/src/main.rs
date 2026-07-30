use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    let (text, set_text) = signal(String::new());
    let (saved, set_saved) = signal(String::new());
    let version = RwSignal::new(0u32);

    Effect::new(move |_| {
        let value = text.get();
        if value.is_empty() {
            return;
        }
        let v = version.get() + 1;
        version.set(v);
        let cloned = value.clone();
        set_timeout(
            move || {
                if version.get() == v {
                    leptos::logging::log!("自动保存: {}", cloned);
                    set_saved.set(cloned);
                }
            },
            std::time::Duration::from_millis(500),
        );
    });

    view! {
        <div style="padding: 1rem;">
            <h2>"防抖提交"</h2>
            <p>"输入内容，500ms 后自动保存:"</p>
            <textarea
                rows="4"
                prop:value={text.get()}
                on:input=move |ev| set_text.set(event_target_value(&ev))
                placeholder="输入内容..."
                style="width: 100%; max-width: 500px;"
            ></textarea>
            <p>"已保存: " {saved.get()}</p>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
