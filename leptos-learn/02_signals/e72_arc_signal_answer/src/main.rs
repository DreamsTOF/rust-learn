use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    let (count, set_count) = arc_signal(0);
    let count_clone = count.clone();
    let double = ArcSignal::derive(move || count_clone.get() * 2);
    let _ = double.get();
    set_count.set(1);
    let _ = count.get();
    view! {
        <p>"ArcSignal 是引用计数、线程安全的信号类型。"</p>
        <p>"适用于需要在不同作用域或线程间共享的场景。"</p>
    }
}

fn main() {
    mount_to_body(Exercise);
}
