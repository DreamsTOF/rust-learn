use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    let (count, set_count) = signal(0);
    let stored = StoredValue::new(0);

    Effect::new(move || {
        let c = count.get();
        let s = *stored.read_value();
        let _ = (c, s);
    });

    set_count.set(1);
    *stored.write_value() = 1;

    view! {
        <p>"Signal 是响应式的——修改会触发依赖它的 effect。"</p>
        <p>"StoredValue 是非响应式的——修改不触发任何 effect。"</p>
    }
}

fn main() {
    mount_to_body(Exercise);
}
