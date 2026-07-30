use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    let (count, _set_count) = signal(42);

    let by_get = count.get();
    let by_with = count.with(|val| format!("with: {}", val));
    let by_read = format!("read: {}", *count.read());

    view! {
        <div>
            <h1>"练习 26: 三种读取方式对比"</h1>
            <p>"count.get() = " {by_get}</p>
            <p>"count.with(|v| ...) = " {by_with}</p>
            <p>"*count.read() = " {by_read}</p>
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(Exercise);
}
