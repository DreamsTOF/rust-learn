use leptos::prelude::*;

fn main() {
    mount_to_body(|| view! { <Exercise/> });
}

#[component]
fn Exercise() -> impl IntoView {
    let (name, set_name) = signal(String::from("Leptos"));
    let upper = Memo::new(move |_| name.get().to_uppercase());

    view! {
        <div>
            <p>"练习 44 (memo_with)"</p>
            <p>"原始值: " {name}</p>
            <p>"upper.get()  = " {upper.get()}</p>
            <p>"upper.with() = " {move || upper.with(|s| s.clone())}</p>
            <p>"长度 (.with): " {move || upper.with(|s| s.len())}</p>
            <button on:click=move |_| set_name.set(format!("{}x", name.get()))>"追加 x"</button>
        </div>
    }
}
