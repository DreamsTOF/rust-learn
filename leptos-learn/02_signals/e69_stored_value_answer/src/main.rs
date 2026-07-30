use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    let stored = StoredValue::new(42);
    let guard = stored.read_value();
    let _value: &i32 = &guard;
    *stored.write_value() = 100;

    view! {
        <p>"StoredValue 是 Copy 的但非响应式——读写都不会追踪或通知订阅者。"</p>
    }
}

fn main() {
    mount_to_body(Exercise);
}
