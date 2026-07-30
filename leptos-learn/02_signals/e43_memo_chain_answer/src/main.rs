use leptos::prelude::*;

fn main() {
    mount_to_body(|| view! { <Exercise/> });
}

#[component]
fn Exercise() -> impl IntoView {
   let (a, set_a) = signal(1);
   let memo1 = Memo::new(move |_| a.get() * 2);
   let memo2 = Memo::new(move |_| memo1.get() + 3);

    view! {
        <div>
            <p>"练习 43 (memo_chain)"</p>
            <p>"a = " {a} "（原始信号）"</p>
            <p>"memo1 = a * 2 = " {memo1}</p>
            <p>"memo2 = memo1 + 3 = " {memo2}</p>
            <button on:click=move |_| set_a.set(a.get() + 1)>"a + 1"</button>
        </div>
    }
}
