use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    let (x, set_x) = signal(1);

    let x_triple_closure = move || x.get() * 3;

    let x_triple_memo = Memo::new(move |_| x.get() * 3);

    view! {
        <div>
            <p>"练习 41: memo_vs_closure"</p>
            <p>"x = " {x}</p>
            <p>"闭包 x*3 = " {x_triple_closure}</p>
            <p>"Memo x*3 = " {x_triple_memo}</p>
            <button on:click=move |_| set_x.update(|v| *v += 1)>"x += 1"</button>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
