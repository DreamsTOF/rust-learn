use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    let (count, set_count) = signal(5);

    let double = Memo::new(move |_| count.get() * 2);

    let double_via_get = double.get();

    view! {
        <div>
            <p>"练习 42: memo_get"</p>
            <p>"count = " {count}</p>
            <p>"double (memo 信号) = " {double}</p>
            <p>"double.get() = " {double_via_get}</p>
            <button on:click=move |_| set_count.update(|v| *v += 1)>"count += 1"</button>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
