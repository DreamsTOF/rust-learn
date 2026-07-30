use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    let (show_a, set_show_a) = signal(true);

    view! {
        <button on:click=move |_| set_show_a.update(|v| *v = !*v)>
            "切换视图"
        </button>
        <Show
            when=move || show_a.get()
            fallback=|| view! {
                <div style="padding: 1rem; background: #e8f5e9;">
                    <p>"这是视图 B"</p>
                    <small>"绿色背景"</small>
                </div>
            }
        >
            <div style="padding: 1rem; background: #e3f2fd;">
                <p>"这是视图 A"</p>
                <small>"蓝色背景"</small>
            </div>
        </Show>
    }
}

fn main() {
    mount_to_body(Exercise);
}
