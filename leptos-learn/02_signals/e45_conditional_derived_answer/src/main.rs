use leptos::prelude::*;

fn main() {
    mount_to_body(|| view! { <Exercise/> });
}

#[component]
fn Exercise() -> impl IntoView {
    let (count, set_count) = signal(0);
    let (toggle, set_toggle) = signal(true);

    let derived = move || {
        if toggle.get() {
            format!("当前值: {}", count.get())
        } else {
            "已关闭".to_string()
        }
    };

    view! {
        <div>
            <p>"练习 45 (conditional_derived)"</p>
            <p>"派生值: " {derived}</p>
            <button on:click=move |_| set_count.set(count.get() + 1)>"+1"</button>
            <button on:click=move |_| set_count.set(0)>"重置"</button>
            <button on:click=move |_| set_toggle.set(!toggle.get())>
                {move || if toggle.get() { "关闭跟踪" } else { "开启跟踪" }}
            </button>
        </div>
    }
}
