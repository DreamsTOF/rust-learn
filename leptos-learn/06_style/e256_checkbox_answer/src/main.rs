use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    let (checked, set_checked) = signal(false);

    view! {
        <div>
            <h2>"练习 256 — 复选框"</h2>
            <label>
                <input type="checkbox"
                    prop:checked={checked}
                    on:change=move|ev| {
                        let input = event_target::<leptos::web_sys::HtmlInputElement>(&ev);
                        set_checked.set(input.checked());
                    }
                />
                " 勾选此项"
            </label>
            <p>{
                move || if checked.get() { "✓ 已勾选" } else { "✗ 未勾选" }
            }</p>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
