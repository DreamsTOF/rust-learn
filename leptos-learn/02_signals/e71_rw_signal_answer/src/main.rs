use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    let count = RwSignal::new(0);
    let _ = count.get();
    count.set(1);
    count.update(|n| *n += 1);
    let (read, write) = count.split();
    let _ = read.get();
    write.set(3);
    view! {
        <p>"RwSignal 通过同一个 handle 同时支持读写操作。"</p>
    }
}

fn main() {
    mount_to_body(Exercise);
}
