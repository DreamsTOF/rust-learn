use leptos::prelude::*;

#[component]
fn ConfirmButton<F>(on_confirm: F) -> impl IntoView
where
    F: Fn() + 'static,
{
    view! {
        <button
            style="background:#e74c3c;color:white;border:none;padding:8px 16px;border-radius:4px;cursor:pointer;"
            on:click=move |_| on_confirm()
        >
            "确认删除"
        </button>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    let (msg, set_msg) = signal("等待操作...".to_string());
    view! {
        <p>{msg}</p>
        <ConfirmButton on_confirm=move || set_msg.set("已确认删除！".to_string()) />
    }
}

fn main() {
    mount_to_body(Exercise);
}
