use leptos::prelude::*;

fn main() {
    mount_to_body(|| {
        let (count, _set_count) = signal(5);

        let double = move || count() * 2;

        view! {
            <p>"练习 38 — 派生闭包 (derived_closure)"</p>
            <p>"count = " {count()} "，double = " {double()}</p>
        }
    });
}
