use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    let (admin, set_admin) = signal(false);
    let (verified, set_verified) = signal(false);

    view! {
        <label>
            <input type="checkbox"
                prop:checked=move || admin.get()
                on:change=move |_| set_admin.update(|v| *v = !*v)
            />
            "管理员"
        </label>
        <label>
            <input type="checkbox"
                prop:checked=move || verified.get()
                on:change=move |_| set_verified.update(|v| *v = !*v)
            />
            "已验证"
        </label>
        <Show when=move || admin.get() && verified.get()>
            <p>"🔒 管理面板（仅管理员且已验证可见）"</p>
        </Show>
    }
}

fn main() {
    mount_to_body(Exercise);
}
