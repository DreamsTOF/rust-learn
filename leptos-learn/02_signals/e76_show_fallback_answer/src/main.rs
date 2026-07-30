use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    let (logged_in, set_logged_in) = signal(false);

    view! {
        <button on:click=move |_| set_logged_in.update(|v| *v = !*v)>
            {move || if logged_in.get() { "退出" } else { "登录" }}
        </button>
        <Show
            when=move || logged_in.get()
            fallback=|| view! { <p>"请先登录"</p> }
        >
            <p>"欢迎回来！"</p>
        </Show>
    }
}

fn main() {
    mount_to_body(Exercise);
}
